use crate::{digest, Digest, Error, Id, Result};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const CONTROLLER_SCOPE: Id = [0; 16];
const DOMAIN: &[u8] = b"VIDA-CTRL-EVENT-v1\0";
const MAX_EVENTS: usize = 4096;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnrollmentIntent {
    pub persona: Id,
    pub device: Id,
    pub key: [u8; 32],
    pub scope: Id,
    pub nonce: Id,
    pub expires_at: u64,
    pub signature: Vec<u8>,
}

impl EnrollmentIntent {
    pub fn new(persona: Id, device: Id, scope: Id, expires_at: u64, key: &SigningKey) -> Self {
        let mut intent = Self {
            persona,
            device,
            key: key.verifying_key().to_bytes(),
            scope,
            nonce: crate::random_id(),
            expires_at,
            signature: vec![],
        };
        intent.signature = key.sign(&intent.bytes()).to_bytes().to_vec();
        intent
    }
    fn bytes(&self) -> Vec<u8> {
        let mut out = b"VIDA-ENROLL-INTENT-v1\0".to_vec();
        out.extend(self.persona);
        out.extend(self.device);
        out.extend(self.key);
        out.extend(self.scope);
        out.extend(self.nonce);
        out.extend(self.expires_at.to_le_bytes());
        out
    }
    pub fn verify(&self, now: u64) -> Result<()> {
        if now > self.expires_at {
            return Err(Error::Stale);
        }
        let key = VerifyingKey::from_bytes(&self.key).map_err(|_| Error::Authentication)?;
        let sig: [u8; 64] = self
            .signature
            .as_slice()
            .try_into()
            .map_err(|_| Error::Encoding)?;
        key.verify(&self.bytes(), &Signature::from_bytes(&sig))
            .map_err(|_| Error::Authentication)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Action {
    Genesis {
        device: Id,
        key: [u8; 32],
        recovery_key: [u8; 32],
    },
    Grant {
        intent: EnrollmentIntent,
        approved_at: u64,
    },
    Revoke {
        device: Id,
        scope: Id,
    },
    RecoveryGrant {
        device: Id,
        key: [u8; 32],
    },
    Rotate {
        recovery_key: [u8; 32],
        bundle_hash: Digest,
    },
    Resolve {
        device: Id,
        scope: Id,
        branches: [Digest; 2],
        key: Option<[u8; 32]>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Event {
    pub persona: Id,
    pub signer: [u8; 32],
    pub sequence: u64,
    pub parents: Vec<Digest>,
    pub epoch: u64,
    pub command: Id,
    pub action: Action,
    pub signature: Vec<u8>,
}

impl Event {
    pub fn signed_bytes(&self) -> Result<Vec<u8>> {
        if self.parents.len() > 64 || !strictly_sorted(&self.parents) {
            return Err(Error::Encoding);
        }
        let mut out = DOMAIN.to_vec();
        out.extend(self.persona);
        out.extend(self.signer);
        out.extend(self.sequence.to_le_bytes());
        out.push(self.parents.len() as u8);
        for p in &self.parents {
            out.extend(p);
        }
        out.extend(self.epoch.to_le_bytes());
        out.extend(self.command);
        match &self.action {
            Action::Genesis {
                device,
                key,
                recovery_key,
            } => {
                out.push(0);
                out.extend(device);
                out.extend(key);
                out.extend(recovery_key);
            }
            Action::Grant {
                intent,
                approved_at,
            } => {
                out.push(1);
                out.extend(intent.bytes());
                out.extend(&intent.signature);
                out.extend(approved_at.to_le_bytes());
            }
            Action::Revoke { device, scope } => {
                out.push(2);
                out.extend(device);
                out.extend(scope);
            }
            Action::RecoveryGrant { device, key } => {
                out.push(3);
                out.extend(device);
                out.extend(key);
            }
            Action::Rotate {
                recovery_key,
                bundle_hash,
            } => {
                out.push(4);
                out.extend(recovery_key);
                out.extend(bundle_hash);
            }
            Action::Resolve {
                device,
                scope,
                branches,
                key,
            } => {
                if branches[0] >= branches[1] {
                    return Err(Error::Encoding);
                }
                out.push(5);
                out.extend(device);
                out.extend(scope);
                for b in branches {
                    out.extend(b);
                }
                match key {
                    Some(k) => {
                        out.push(1);
                        out.extend(k);
                    }
                    None => out.push(0),
                }
            }
        }
        Ok(out)
    }

    pub fn id(&self) -> Result<Digest> {
        let mut bytes = self.signed_bytes()?;
        bytes.extend(&self.signature);
        Ok(digest(&bytes))
    }

    pub fn verify(&self) -> Result<()> {
        let key = VerifyingKey::from_bytes(&self.signer).map_err(|_| Error::Authentication)?;
        let sig: [u8; 64] = self
            .signature
            .as_slice()
            .try_into()
            .map_err(|_| Error::Encoding)?;
        key.verify(&self.signed_bytes()?, &Signature::from_bytes(&sig))
            .map_err(|_| Error::Authentication)
    }
}

fn strictly_sorted(ids: &[Digest]) -> bool {
    ids.windows(2).all(|pair| pair[0] < pair[1])
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Access {
    Denied,
    Verified([u8; 32]),
    Provisional([u8; 32]),
    Conflict,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct History {
    pub persona: Id,
    pub events: BTreeMap<Digest, Event>,
}

impl History {
    pub fn new(persona: Id, device: Id, owner: &SigningKey, recovery_key: [u8; 32]) -> Self {
        let mut event = Event {
            persona,
            signer: owner.verifying_key().to_bytes(),
            sequence: 0,
            parents: vec![],
            epoch: 0,
            command: crate::random_id(),
            action: Action::Genesis {
                device,
                key: owner.verifying_key().to_bytes(),
                recovery_key,
            },
            signature: vec![0; 64],
        };
        event.signature = owner
            .sign(&event.signed_bytes().expect("genesis encoding"))
            .to_bytes()
            .to_vec();
        let mut events = BTreeMap::new();
        events.insert(event.id().expect("genesis id"), event);
        Self { persona, events }
    }

    pub fn frontier(&self) -> Vec<Digest> {
        let referenced: BTreeSet<Digest> = self
            .events
            .values()
            .flat_map(|e| e.parents.iter().copied())
            .collect();
        self.events
            .keys()
            .filter(|id| !referenced.contains(*id))
            .copied()
            .collect()
    }

    pub fn frontier_hash(&self) -> Digest {
        let mut bytes = b"VIDA-CTRL-FRONTIER-v1\0".to_vec();
        for id in self.frontier() {
            bytes.extend(id);
        }
        digest(&bytes)
    }

    pub fn merge(&mut self, other: &Self) -> Result<()> {
        let mut candidate = self.clone();
        candidate.merge_inner(other)?;
        *self = candidate;
        Ok(())
    }

    fn merge_inner(&mut self, other: &Self) -> Result<()> {
        if self.persona != other.persona {
            return Err(Error::Authority);
        }
        if other
            .events
            .iter()
            .any(|(id, event)| event.id().ok() != Some(*id))
        {
            return Err(Error::Authentication);
        }
        let mut pending: BTreeMap<_, _> = other
            .events
            .iter()
            .filter(|(id, _)| !self.events.contains_key(*id))
            .map(|(id, e)| (*id, e.clone()))
            .collect();
        while !pending.is_empty() {
            let ready: Vec<_> = pending
                .iter()
                .filter(|(_, e)| e.parents.iter().all(|p| self.events.contains_key(p)))
                .map(|(id, _)| *id)
                .collect();
            if ready.is_empty() {
                return Err(Error::Authority);
            }
            for id in ready {
                self.insert(pending.remove(&id).expect("ready"))?;
            }
        }
        Ok(())
    }

    pub fn insert(&mut self, event: Event) -> Result<Digest> {
        if self.events.len() >= MAX_EVENTS || event.persona != self.persona {
            return Err(Error::Encoding);
        }
        event.verify()?;
        let id = event.id()?;
        if self.events.contains_key(&id) || self.events.values().any(|e| e.command == event.command)
        {
            return Err(Error::Replay);
        }
        if event.parents.iter().any(|p| !self.events.contains_key(p)) {
            return Err(Error::Authority);
        }
        if matches!(event.action, Action::Genesis { .. }) || event.parents.is_empty() {
            return Err(Error::Authority);
        }
        let parent_view = self.closure(&event.parents);
        let sequence = parent_view
            .values()
            .filter(|e| e.signer == event.signer)
            .map(|e| e.sequence)
            .max()
            .map_or(0, |n| n + 1);
        if event.sequence != sequence {
            return Err(Error::Replay);
        }
        let epoch = self.epoch_at(&parent_view)?;
        if event.epoch != epoch {
            return Err(Error::Stale);
        }
        if let Action::Grant {
            intent,
            approved_at,
        } = &event.action
        {
            if intent.persona != self.persona {
                return Err(Error::Authority);
            }
            intent.verify(*approved_at)?;
            if self.events.iter().any(|(prior_id, e)| matches!(&e.action, Action::Grant { intent: old, .. } if old.nonce == intent.nonce && (old != intent || parent_view.contains_key(prior_id)))) {
                return Err(Error::Replay);
            }
            if parent_view.values().any(|e| {
                e.bound_key()
                    .is_some_and(|(device, key)| device != intent.device && key == intent.key)
            }) {
                return Err(Error::Authority);
            }
        }
        if let Action::RecoveryGrant { device, key } = &event.action {
            if parent_view
                .values()
                .any(|e| e.subject().is_some_and(|(existing, _)| existing == *device))
                || parent_view.values().any(|e| {
                    e.bound_key()
                        .is_some_and(|(existing, bound)| existing != *device && bound == *key)
                })
            {
                return Err(Error::Authority);
            }
        }
        let recovery = matches!(event.action, Action::RecoveryGrant { .. });
        if recovery {
            if self.recovery_key_at(&parent_view)? != event.signer {
                return Err(Error::Authority);
            }
        } else if !matches!(self.access_at(&parent_view, self.signer_device(&parent_view, event.signer)?, CONTROLLER_SCOPE), Access::Verified(k) if k == event.signer)
        {
            return Err(Error::Authority);
        }
        if let Action::Resolve {
            device,
            scope,
            branches,
            key,
        } = &event.action
        {
            let mut eligible_keys = Vec::new();
            for b in branches {
                let branch = parent_view.get(b).ok_or(Error::Authority)?;
                if branch.subject() != Some((*device, *scope))
                    || parent_view.iter().any(|(other_id, other)| {
                        other_id != b
                            && other.subject() == Some((*device, *scope))
                            && self.ancestor(*b, other_id)
                    })
                {
                    return Err(Error::Authority);
                }
                if let Some((_, branch_key)) = branch.bound_key() {
                    eligible_keys.push(branch_key);
                }
            }
            if key.is_some_and(|chosen| !eligible_keys.contains(&chosen)) {
                return Err(Error::Authority);
            }
        }
        self.events.insert(id, event);
        Ok(id)
    }

    pub fn append(&mut self, signer: &SigningKey, command: Id, action: Action) -> Result<Digest> {
        let parents = self.frontier();
        let closure = self.closure(&parents);
        let sequence = closure
            .values()
            .filter(|e| e.signer == signer.verifying_key().to_bytes())
            .map(|e| e.sequence)
            .max()
            .map_or(0, |n| n + 1);
        let epoch = self.epoch_at(&closure)?;
        let mut event = Event {
            persona: self.persona,
            signer: signer.verifying_key().to_bytes(),
            sequence,
            parents,
            epoch,
            command,
            action,
            signature: vec![0; 64],
        };
        event.signature = signer.sign(&event.signed_bytes()?).to_bytes().to_vec();
        self.insert(event)
    }

    pub fn access(&self, device: Id, scope: Id) -> Access {
        self.access_at(&self.events, device, scope)
    }

    fn access_at(&self, view: &BTreeMap<Digest, Event>, device: Id, scope: Id) -> Access {
        let mut heads: Vec<(&Digest, &Event)> = view
            .iter()
            .filter(|(_, e)| e.subject() == Some((device, scope)))
            .collect();
        heads.retain(|(id, _)| {
            !view.iter().any(|(other_id, other)| {
                other_id != *id
                    && other.subject() == Some((device, scope))
                    && self.ancestor(**id, other_id)
            })
        });
        if heads.len() > 1 {
            return Access::Conflict;
        }
        let access = match heads.first().map(|(_, e)| &e.action) {
            Some(Action::Genesis { key, .. }) => Access::Verified(*key),
            Some(Action::Grant { intent, .. }) => Access::Verified(intent.key),
            Some(Action::RecoveryGrant { key, .. }) => Access::Provisional(*key),
            Some(Action::Resolve { key: Some(key), .. }) => Access::Verified(*key),
            _ => Access::Denied,
        };
        match access {
            Access::Verified(key) | Access::Provisional(key)
                if view.values().any(|e| {
                    e.bound_key().is_some_and(|(bound_device, bound_key)| {
                        bound_device != device && bound_key == key
                    })
                }) =>
            {
                Access::Conflict
            }
            other => other,
        }
    }

    fn signer_device(&self, view: &BTreeMap<Digest, Event>, signer: [u8; 32]) -> Result<Id> {
        let devices: BTreeSet<Id> = view
            .values()
            .filter_map(|e| e.bound_key())
            .filter_map(|(device, key)| (key == signer).then_some(device))
            .collect();
        if devices.len() != 1 {
            return Err(Error::Authority);
        }
        devices.iter().next().copied().ok_or(Error::Authority)
    }

    fn closure(&self, roots: &[Digest]) -> BTreeMap<Digest, Event> {
        let mut out = BTreeMap::new();
        let mut stack = roots.to_vec();
        while let Some(id) = stack.pop() {
            if let Some(e) = self.events.get(&id) {
                if out.insert(id, e.clone()).is_none() {
                    stack.extend(&e.parents);
                }
            }
        }
        out
    }

    fn ancestor(&self, ancestor: Digest, descendant: &Digest) -> bool {
        self.closure(&[*descendant]).contains_key(&ancestor) && ancestor != *descendant
    }

    fn epoch_at(&self, view: &BTreeMap<Digest, Event>) -> Result<u64> {
        let rotations = self.rotation_heads(view);
        if rotations.len() > 1 {
            return Err(Error::Authority);
        }
        Ok(rotations.first().map_or(0, |(_, e)| e.epoch + 1))
    }

    fn recovery_key_at(&self, view: &BTreeMap<Digest, Event>) -> Result<[u8; 32]> {
        let rotations = self.rotation_heads(view);
        if rotations.len() > 1 {
            return Err(Error::Authority);
        }
        if let Some((_, e)) = rotations.first() {
            if let Action::Rotate { recovery_key, .. } = e.action {
                return Ok(recovery_key);
            }
        }
        view.values()
            .find_map(|e| {
                if let Action::Genesis { recovery_key, .. } = e.action {
                    Some(recovery_key)
                } else {
                    None
                }
            })
            .ok_or(Error::Authority)
    }

    fn rotation_heads<'a>(
        &self,
        view: &'a BTreeMap<Digest, Event>,
    ) -> Vec<(&'a Digest, &'a Event)> {
        view.iter()
            .filter(|(id, e)| {
                matches!(e.action, Action::Rotate { .. })
                    && !view.iter().any(|(other_id, other)| {
                        other_id != *id
                            && matches!(other.action, Action::Rotate { .. })
                            && self.ancestor(**id, other_id)
                    })
            })
            .collect()
    }

    pub fn epoch(&self) -> Result<u64> {
        self.epoch_at(&self.events)
    }
    pub fn recovery_key(&self) -> Result<[u8; 32]> {
        self.recovery_key_at(&self.events)
    }

    pub fn validate(&self) -> Result<()> {
        if self.events.len() > MAX_EVENTS {
            return Err(Error::Encoding);
        }
        let roots: Vec<_> = self
            .events
            .values()
            .filter(|e| e.parents.is_empty())
            .collect();
        if roots.len() != 1 {
            return Err(Error::Authority);
        }
        let root = roots[0];
        root.verify()?;
        if root.persona != self.persona
            || root.sequence != 0
            || root.epoch != 0
            || !matches!(root.action, Action::Genesis { key, .. } if key == root.signer)
        {
            return Err(Error::Authority);
        }
        let mut rebuilt = Self {
            persona: self.persona,
            events: BTreeMap::from([(root.id()?, root.clone())]),
        };
        rebuilt.merge(self)?;
        if rebuilt.events.len() != self.events.len()
            || self.events.iter().any(|(id, e)| e.id().ok() != Some(*id))
        {
            return Err(Error::Authentication);
        }
        Ok(())
    }
}

impl Event {
    fn bound_key(&self) -> Option<(Id, [u8; 32])> {
        match &self.action {
            Action::Genesis { device, key, .. } | Action::RecoveryGrant { device, key } => {
                Some((*device, *key))
            }
            Action::Grant { intent, .. } => Some((intent.device, intent.key)),
            Action::Resolve {
                device,
                key: Some(key),
                ..
            } => Some((*device, *key)),
            _ => None,
        }
    }

    fn subject(&self) -> Option<(Id, Id)> {
        match self.action {
            Action::Genesis { device, .. } | Action::RecoveryGrant { device, .. } => {
                Some((device, CONTROLLER_SCOPE))
            }
            Action::Grant { ref intent, .. } => Some((intent.device, intent.scope)),
            Action::Revoke { device, scope } | Action::Resolve { device, scope, .. } => {
                Some((device, scope))
            }
            Action::Rotate { .. } => None,
        }
    }
}
