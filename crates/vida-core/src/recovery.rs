use crate::controller::{Access, Action, History, CONTROLLER_SCOPE};
use crate::{digest, random_id, Digest, Error, Id, Result};
use bincode::Options;
use chacha20poly1305::aead::{Aead, KeyInit, Payload};
use chacha20poly1305::{XChaCha20Poly1305, XNonce};
use ed25519_dalek::SigningKey;
use hkdf::Hkdf;
use rand_core::RngCore;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use zeroize::{Zeroize, Zeroizing};

const BUNDLE_MAGIC: &[u8; 5] = b"VDBN1";
const NOTE_MAGIC: &[u8; 5] = b"VDNT1";
const VERSION: u8 = 1;
const BUNDLE_MAX: usize = 4 * 1024 * 1024;
const NOTE_MAX: usize = 1024 * 1024;

pub struct Secret(pub Zeroizing<[u8; 32]>);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Export {
    pub bytes: Vec<u8>,
    pub frontier: Digest,
    pub coverage: Vec<Coverage>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Coverage {
    pub id: Id,
    pub frontier: Digest,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Note {
    pub id: Id,
    pub content: Vec<u8>,
    pub frontier: Digest,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Authority {
    Provisional,
    Verified,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RotationProofStatus {
    Pending,
    Passed,
    Critical,
}

pub struct RotationHealth {
    history: History,
    bundle_hash: Digest,
    pub status: RotationProofStatus,
}

impl RotationHealth {
    pub fn after_commit(history: &History, bundle: &[u8]) -> Result<Self> {
        history.validate()?;
        let bundle_hash = digest(bundle);
        let current_key = history.recovery_key()?;
        if !history.events.values().any(|event| matches!(event.action, Action::Rotate { recovery_key, bundle_hash: committed } if recovery_key == current_key && committed == bundle_hash)) {
            return Err(Error::Stale);
        }
        Ok(Self {
            history: history.clone(),
            bundle_hash,
            status: RotationProofStatus::Pending,
        })
    }

    pub fn test_restore(
        &mut self,
        secret: &Secret,
        bundle: &[u8],
        resource: Option<&[u8]>,
    ) -> Result<Restored> {
        let result = if digest(bundle) != self.bundle_hash {
            Err(Error::Stale)
        } else {
            restore(
                secret,
                bundle,
                resource,
                self.history.persona,
                Some(&self.history),
            )
        };
        match &result {
            Ok(_) if self.status == RotationProofStatus::Pending => {
                self.status = RotationProofStatus::Passed
            }
            Err(_) => self.status = RotationProofStatus::Critical,
            _ => {}
        }
        result
    }
}

pub struct Restored {
    pub device: Id,
    pub signing: SigningKey,
    pub history: History,
    pub note: Option<Note>,
    pub content_frontier: Option<Digest>,
    pub authority: Authority,
    pub data_keys_restored: bool,
}

#[derive(Serialize, Deserialize)]
struct BundlePlain {
    history: History,
    frontier: Digest,
    epoch: u64,
    data_key: [u8; 32],
    recovery_seed: [u8; 32],
    coverage: Vec<Coverage>,
    pending_rotation: bool,
}

impl Drop for BundlePlain {
    fn drop(&mut self) {
        self.data_key.zeroize();
        self.recovery_seed.zeroize();
    }
}

pub struct Kit {
    pub secret: Secret,
    recovery: SigningKey,
    data_key: Zeroizing<[u8; 32]>,
}

impl Kit {
    pub fn new(data_key: Option<[u8; 32]>) -> Self {
        let mut secret = [0; 32];
        let mut seed = Zeroizing::new([0; 32]);
        let mut data = Zeroizing::new(data_key.unwrap_or([0; 32]));
        rand_core::OsRng.fill_bytes(&mut secret);
        rand_core::OsRng.fill_bytes(&mut *seed);
        if data_key.is_none() {
            rand_core::OsRng.fill_bytes(&mut *data);
        }
        Self {
            secret: Secret(Zeroizing::new(secret)),
            recovery: SigningKey::from_bytes(&seed),
            data_key: data,
        }
    }

    pub fn recovery_key(&self) -> [u8; 32] {
        self.recovery.verifying_key().to_bytes()
    }

    pub fn export_bundle(&self, history: &History, coverage: Vec<Coverage>) -> Result<Export> {
        let mut salt = [0; 16];
        let mut nonce = [0; 24];
        rand_core::OsRng.fill_bytes(&mut salt);
        rand_core::OsRng.fill_bytes(&mut nonce);
        self.export_bundle_with_entropy(history, coverage, salt, nonce)
    }

    fn export_bundle_with_entropy(
        &self,
        history: &History,
        mut coverage: Vec<Coverage>,
        salt: [u8; 16],
        nonce: [u8; 24],
    ) -> Result<Export> {
        history.validate()?;
        coverage.sort();
        if coverage.len() > 1 || coverage.windows(2).any(|pair| pair[0].id == pair[1].id) {
            return Err(Error::Encoding);
        }
        let plain = BundlePlain {
            history: history.clone(),
            frontier: history.frontier_hash(),
            epoch: history.epoch()?,
            data_key: *self.data_key,
            recovery_seed: self.recovery.to_bytes(),
            coverage: coverage.clone(),
            pending_rotation: history.recovery_key()? != self.recovery_key(),
        };
        let bytes = Zeroizing::new(codec().serialize(&plain).map_err(|_| Error::Encoding)?);
        if bytes.len() > BUNDLE_MAX {
            return Err(Error::Encoding);
        }
        let key = derive(&self.secret.0, &salt, b"VIDA-bundle-aead-v1")?;
        let mut header = BUNDLE_MAGIC.to_vec();
        header.push(VERSION);
        header.extend(history.persona);
        header.extend(salt);
        header.extend(nonce);
        header.extend(((bytes.len() + 16) as u32).to_le_bytes());
        let cipher = XChaCha20Poly1305::new((&*key).into());
        let encrypted = cipher
            .encrypt(
                XNonce::from_slice(&nonce),
                Payload {
                    msg: &bytes,
                    aad: &header,
                },
            )
            .map_err(|_| Error::Authentication)?;
        header.extend(encrypted);
        Ok(Export {
            bytes: header,
            frontier: plain.frontier,
            coverage,
        })
    }

    pub fn export_note(&self, persona: Id, history: &History, note: &Note) -> Result<Export> {
        let mut nonce = [0; 24];
        rand_core::OsRng.fill_bytes(&mut nonce);
        self.export_note_with_nonce(persona, history, note, nonce)
    }

    fn export_note_with_nonce(
        &self,
        persona: Id,
        history: &History,
        note: &Note,
        nonce: [u8; 24],
    ) -> Result<Export> {
        history.validate()?;
        if note.content.len() > NOTE_MAX || history.persona != persona {
            return Err(Error::Encoding);
        }
        let frontier = note.frontier;
        let key = derive(&self.data_key, &persona, b"VIDA-note-aead-v1")?;
        let mut header = NOTE_MAGIC.to_vec();
        header.push(VERSION);
        header.extend(persona);
        header.extend(note.id);
        header.extend(frontier);
        header.extend(nonce);
        header.extend(((note.content.len() + 16) as u32).to_le_bytes());
        let cipher = XChaCha20Poly1305::new((&*key).into());
        let encrypted = cipher
            .encrypt(
                XNonce::from_slice(&nonce),
                Payload {
                    msg: &note.content,
                    aad: &header,
                },
            )
            .map_err(|_| Error::Authentication)?;
        header.extend(encrypted);
        Ok(Export {
            bytes: header,
            frontier,
            coverage: vec![Coverage {
                id: note.id,
                frontier,
            }],
        })
    }

    pub fn prepare_rotation(
        &self,
        history: &History,
        coverage: Vec<Coverage>,
    ) -> Result<PreparedRotation> {
        Self::repair_from_trusted(*self.data_key, history, coverage)
    }

    pub fn repair_from_trusted(
        data_key: [u8; 32],
        history: &History,
        coverage: Vec<Coverage>,
    ) -> Result<PreparedRotation> {
        let next = Kit::new(Some(data_key));
        let export = next.export_bundle(history, coverage)?;
        Ok(PreparedRotation {
            next,
            export,
            expected_frontier: history.frontier_hash(),
            expected_epoch: history.epoch()?,
            custody_confirmed: false,
            verified_hash: None,
        })
    }
}

pub struct PreparedRotation {
    pub next: Kit,
    pub export: Export,
    expected_frontier: Digest,
    expected_epoch: u64,
    custody_confirmed: bool,
    verified_hash: Option<Digest>,
}

impl PreparedRotation {
    pub fn confirm_custody(&mut self) {
        self.custody_confirmed = true;
    }
    pub fn verify_export(&mut self, readback: &[u8]) -> Result<()> {
        if readback != self.export.bytes {
            return Err(Error::Verification);
        }
        let plain = open_bundle(&self.next.secret, readback, None)?;
        if plain.frontier != self.expected_frontier || plain.epoch != self.expected_epoch {
            return Err(Error::Verification);
        }
        self.verified_hash = Some(digest(readback));
        Ok(())
    }
    pub fn commit(&self, history: &mut History, signer: &SigningKey) -> Result<Digest> {
        if !self.custody_confirmed || self.verified_hash != Some(digest(&self.export.bytes)) {
            return Err(Error::Verification);
        }
        if history.frontier_hash() != self.expected_frontier
            || history.epoch()? != self.expected_epoch
        {
            return Err(Error::Stale);
        }
        history.append(
            signer,
            random_id(),
            Action::Rotate {
                recovery_key: self.next.recovery_key(),
                bundle_hash: digest(&self.export.bytes),
            },
        )
    }
}

pub fn restore(
    secret: &Secret,
    bundle: &[u8],
    resource: Option<&[u8]>,
    expected_persona: Id,
    known: Option<&History>,
) -> Result<Restored> {
    let plain = open_bundle(secret, bundle, Some(expected_persona))?;
    if plain.pending_rotation && known.is_none() {
        return Err(Error::Stale);
    }
    if let Some(current) = known {
        current.validate()?;
        let genesis = |history: &History| {
            history
                .events
                .iter()
                .find(|(_, event)| event.parents.is_empty())
                .map(|(id, _)| *id)
        };
        if current.persona != expected_persona
            || genesis(current) != genesis(&plain.history)
            || current.recovery_key()?
                != SigningKey::from_bytes(&plain.recovery_seed)
                    .verifying_key()
                    .to_bytes()
        {
            return Err(Error::Stale);
        }
        if plain.pending_rotation && !current.events.values().any(|e| matches!(e.action, Action::Rotate { bundle_hash, .. } if bundle_hash == digest(bundle))) { return Err(Error::Stale); }
    }
    let (note, content_frontier) = match resource {
        Some(bytes) => {
            let (note, frontier) =
                open_note(&plain.data_key, expected_persona, bytes, &plain.coverage)?;
            (Some(note), Some(frontier))
        }
        None if plain.coverage.is_empty() => (None, None),
        None => return Err(Error::MissingResource),
    };
    let mut signing_seed = Zeroizing::new([0; 32]);
    rand_core::OsRng.fill_bytes(&mut *signing_seed);
    let signing = SigningKey::from_bytes(&signing_seed);
    let device = random_id();
    let mut history = known.cloned().unwrap_or_else(|| plain.history.clone());
    let recovery = SigningKey::from_bytes(&plain.recovery_seed);
    history.append(
        &recovery,
        random_id(),
        Action::RecoveryGrant {
            device,
            key: signing.verifying_key().to_bytes(),
        },
    )?;
    debug_assert!(matches!(
        history.access(device, CONTROLLER_SCOPE),
        Access::Provisional(_)
    ));
    Ok(Restored {
        device,
        signing,
        history,
        note,
        content_frontier,
        authority: Authority::Provisional,
        data_keys_restored: true,
    })
}

fn open_bundle(secret: &Secret, bytes: &[u8], expected_persona: Option<Id>) -> Result<BundlePlain> {
    const HEADER: usize = 5 + 1 + 16 + 16 + 24 + 4;
    if bytes.len() < HEADER + 16
        || bytes.len() > BUNDLE_MAX + HEADER + 16
        || &bytes[..5] != BUNDLE_MAGIC
        || bytes[5] != VERSION
    {
        return Err(Error::Encoding);
    }
    let persona: Id = bytes[6..22].try_into().map_err(|_| Error::Encoding)?;
    if expected_persona.is_some_and(|p| p != persona) {
        return Err(Error::Authority);
    }
    let len = u32::from_le_bytes(bytes[62..66].try_into().map_err(|_| Error::Encoding)?) as usize;
    if len != bytes.len() - HEADER {
        return Err(Error::Encoding);
    }
    let salt: [u8; 16] = bytes[22..38].try_into().map_err(|_| Error::Encoding)?;
    let key = derive(&secret.0, &salt, b"VIDA-bundle-aead-v1")?;
    let cipher = XChaCha20Poly1305::new((&*key).into());
    let mut decrypted = Zeroizing::new(
        cipher
            .decrypt(
                XNonce::from_slice(&bytes[38..62]),
                Payload {
                    msg: &bytes[HEADER..],
                    aad: &bytes[..HEADER],
                },
            )
            .map_err(|_| Error::Authentication)?,
    );
    let plain: BundlePlain = codec()
        .deserialize(&decrypted)
        .map_err(|_| Error::Encoding)?;
    decrypted.clear();
    if plain.history.persona != persona
        || plain.frontier != plain.history.frontier_hash()
        || plain.epoch != plain.history.epoch()?
    {
        return Err(Error::Authentication);
    }
    plain.history.validate()?;
    if plain.coverage.len() > 1
        || plain
            .coverage
            .windows(2)
            .any(|pair| pair[0].id >= pair[1].id)
    {
        return Err(Error::Encoding);
    }
    if !plain.pending_rotation
        && plain.history.recovery_key()?
            != SigningKey::from_bytes(&plain.recovery_seed)
                .verifying_key()
                .to_bytes()
    {
        return Err(Error::Authority);
    }
    Ok(plain)
}

fn open_note(
    data_key: &[u8; 32],
    persona: Id,
    bytes: &[u8],
    coverage: &[Coverage],
) -> Result<(Note, Digest)> {
    const HEADER: usize = 5 + 1 + 16 + 16 + 32 + 24 + 4;
    if bytes.len() < HEADER + 16
        || bytes.len() > NOTE_MAX + HEADER + 16
        || &bytes[..5] != NOTE_MAGIC
        || bytes[5] != VERSION
        || bytes[6..22] != persona
    {
        return Err(Error::Encoding);
    }
    let id: Id = bytes[22..38].try_into().map_err(|_| Error::Encoding)?;
    let frontier: Digest = bytes[38..70].try_into().map_err(|_| Error::Encoding)?;
    if !coverage
        .iter()
        .any(|c| c.id == id && c.frontier == frontier)
    {
        return Err(Error::MissingResource);
    }
    let len = u32::from_le_bytes(bytes[94..98].try_into().map_err(|_| Error::Encoding)?) as usize;
    if len != bytes.len() - HEADER {
        return Err(Error::Encoding);
    }
    let key = derive(data_key, &persona, b"VIDA-note-aead-v1")?;
    let cipher = XChaCha20Poly1305::new((&*key).into());
    let content = cipher
        .decrypt(
            XNonce::from_slice(&bytes[70..94]),
            Payload {
                msg: &bytes[HEADER..],
                aad: &bytes[..HEADER],
            },
        )
        .map_err(|_| Error::Authentication)?;
    Ok((
        Note {
            id,
            content,
            frontier,
        },
        frontier,
    ))
}

fn derive(secret: &[u8; 32], salt: &[u8], label: &[u8]) -> Result<Zeroizing<[u8; 32]>> {
    let mut key = Zeroizing::new([0; 32]);
    Hkdf::<Sha256>::new(Some(salt), secret)
        .expand(label, &mut *key)
        .map_err(|_| Error::Encoding)?;
    Ok(key)
}

fn codec() -> impl Options {
    bincode::DefaultOptions::new()
        .with_fixint_encoding()
        .with_little_endian()
        .with_limit(BUNDLE_MAX as u64)
        .reject_trailing_bytes()
}

#[cfg(test)]
mod wire_tests {
    use super::*;
    use crate::controller::Event;
    use ed25519_dalek::Signer;
    use std::collections::BTreeMap;

    #[test]
    fn golden_bundle_and_note_v1() {
        let owner = SigningKey::from_bytes(&[7; 32]);
        let recovery = SigningKey::from_bytes(&[8; 32]);
        let persona = [0x11; 16];
        let mut root = Event {
            persona,
            signer: owner.verifying_key().to_bytes(),
            sequence: 0,
            parents: vec![],
            epoch: 0,
            command: [0x22; 16],
            action: Action::Genesis {
                device: [0x33; 16],
                key: owner.verifying_key().to_bytes(),
                recovery_key: recovery.verifying_key().to_bytes(),
            },
            signature: vec![0; 64],
        };
        root.signature = owner
            .sign(&root.signed_bytes().unwrap())
            .to_bytes()
            .to_vec();
        let history = History {
            persona,
            events: BTreeMap::from([(root.id().unwrap(), root)]),
        };
        let kit = Kit {
            secret: Secret(Zeroizing::new([9; 32])),
            recovery,
            data_key: Zeroizing::new([10; 32]),
        };
        let note = Note {
            id: [11; 16],
            content: b"golden".to_vec(),
            frontier: [12; 32],
        };
        let bundle = kit
            .export_bundle_with_entropy(
                &history,
                vec![Coverage {
                    id: note.id,
                    frontier: note.frontier,
                }],
                [13; 16],
                [14; 24],
            )
            .unwrap();
        let resource = kit
            .export_note_with_nonce(persona, &history, &note, [15; 24])
            .unwrap();
        let bundle_hex: String = bundle.bytes.iter().map(|b| format!("{b:02x}")).collect();
        let resource_hex: String = resource.bytes.iter().map(|b| format!("{b:02x}")).collect();
        assert_eq!(
            bundle_hex,
            include_str!("../tests/fixtures/bundle-v1.hex").trim()
        );
        assert_eq!(
            resource_hex,
            include_str!("../tests/fixtures/note-v1.hex")
                .split_whitespace()
                .collect::<String>()
        );
        assert_eq!(
            open_bundle(&kit.secret, &bundle.bytes, Some(persona))
                .unwrap()
                .frontier,
            history.frontier_hash()
        );
        assert_eq!(
            open_note(&kit.data_key, persona, &resource.bytes, &bundle.coverage)
                .unwrap()
                .0,
            note
        );
    }
}
