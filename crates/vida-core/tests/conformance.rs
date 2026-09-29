use ed25519_dalek::SigningKey;
use rand_core::RngCore;
use serde::Serialize;
use std::fs;
use vida_core::controller::{Access, Action, EnrollmentIntent, Event, History, CONTROLLER_SCOPE};
use vida_core::recovery::{
    restore, Authority, Coverage, Kit, Note, RotationHealth, RotationProofStatus,
};
use vida_core::storage::{CrashCut, MemoryStore, StateStore};
use vida_core::{random_id, Error, Id, Result};

fn key() -> SigningKey {
    let mut bytes = [0; 32];
    rand_core::OsRng.fill_bytes(&mut bytes);
    SigningKey::from_bytes(&bytes)
}
fn fixture() -> (Id, Id, SigningKey, Kit, History, Note) {
    let persona = random_id();
    let device = random_id();
    let owner = key();
    let kit = Kit::new(None);
    let history = History::new(persona, device, &owner, kit.recovery_key());
    let note = Note {
        id: random_id(),
        content: b"note at verified frontier".to_vec(),
        frontier: [0x66; 32],
    };
    (persona, device, owner, kit, history, note)
}
fn note_coverage(note: &Note) -> Vec<Coverage> {
    vec![Coverage {
        id: note.id,
        frontier: note.frontier,
    }]
}
fn grant(
    history: &mut History,
    owner: &SigningKey,
    device: Id,
    scope: Id,
    target: &SigningKey,
) -> Result<()> {
    let intent = EnrollmentIntent::new(history.persona, device, scope, 1000, target);
    history.append(
        owner,
        random_id(),
        Action::Grant {
            intent,
            approved_at: 1,
        },
    )?;
    Ok(())
}
fn check(condition: bool) -> Result<()> {
    if condition {
        Ok(())
    } else {
        Err(Error::Verification)
    }
}

fn enrollment() -> Result<()> {
    let (persona, _, owner, _, mut h, _) = fixture();
    let device = random_id();
    let target = key();
    let scope = random_id();
    let intent = EnrollmentIntent::new(persona, device, scope, 10, &target);
    check(intent.verify(11) == Err(Error::Stale))?;
    let mut foreign = persona;
    foreign[0] ^= 1;
    check(matches!(
        h.append(
            &owner,
            random_id(),
            Action::Grant {
                intent: EnrollmentIntent::new(foreign, device, scope, 10, &target),
                approved_at: 1,
            }
        ),
        Err(Error::Authority)
    ))?;
    h.append(
        &owner,
        random_id(),
        Action::Grant {
            intent: intent.clone(),
            approved_at: 1,
        },
    )?;
    check(h.access(device, scope) == Access::Verified(target.verifying_key().to_bytes()))?;
    check(
        h.append(
            &owner,
            random_id(),
            Action::Grant {
                intent,
                approved_at: 1,
            },
        ) == Err(Error::Replay),
    )?;
    check(h.persona == persona)
}

fn fresh_restore() -> Result<()> {
    let (persona, device, owner, kit, h, note) = fixture();
    let bundle = kit.export_bundle(&h, note_coverage(&note))?;
    let resource = kit.export_note(persona, &h, &note)?;
    let restored = restore(
        &kit.secret,
        &bundle.bytes,
        Some(&resource.bytes),
        persona,
        None,
    )?;
    check(restored.note == Some(note))?;
    check(restored.content_frontier == Some(resource.frontier))?;
    check(
        restored.device != device
            && restored.authority == Authority::Provisional
            && restored.data_keys_restored
            && restored.signing.verifying_key() != owner.verifying_key()
            && restored.signing.verifying_key().to_bytes() != kit.recovery_key(),
    )?;
    check(matches!(
        restored.history.access(restored.device, CONTROLLER_SCOPE),
        Access::Provisional(_)
    ))
}

fn missing_parts() -> Result<()> {
    let (persona, _, _, kit, h, note) = fixture();
    let bundle = kit.export_bundle(&h, note_coverage(&note))?;
    check(matches!(
        restore(&kit.secret, &bundle.bytes, None, persona, None),
        Err(Error::MissingResource)
    ))?;
    let persona_only = kit.export_bundle(&h, vec![])?;
    check(
        restore(&kit.secret, &persona_only.bytes, None, persona, None)?
            .note
            .is_none(),
    )?;
    check(matches!(
        restore(&kit.secret, &[], None, persona, None),
        Err(Error::Encoding)
    ))
}

fn wrong_secret() -> Result<()> {
    let (persona, _, _, kit, h, note) = fixture();
    let other = Kit::new(None);
    let bundle = kit.export_bundle(&h, note_coverage(&note))?;
    check(matches!(
        restore(&other.secret, &bundle.bytes, None, persona, None),
        Err(Error::Authentication)
    ))
}

fn tamper_version() -> Result<()> {
    let (persona, _, _, kit, h, note) = fixture();
    let bundle = kit.export_bundle(&h, note_coverage(&note))?;
    let resource = kit.export_note(persona, &h, &note)?;
    let mut bad = bundle.bytes.clone();
    *bad.last_mut().unwrap() ^= 1;
    check(matches!(
        restore(&kit.secret, &bad, None, persona, None),
        Err(Error::Authentication)
    ))?;
    let mut version = bundle.bytes.clone();
    version[5] = 2;
    check(matches!(
        restore(&kit.secret, &version, None, persona, None),
        Err(Error::Encoding)
    ))?;
    let mut foreign = persona;
    foreign[0] ^= 1;
    check(matches!(
        restore(&kit.secret, &bundle.bytes, None, foreign, None),
        Err(Error::Authority)
    ))?;
    let mut bad_resource = resource.bytes.clone();
    *bad_resource.last_mut().unwrap() ^= 1;
    check(matches!(
        restore(
            &kit.secret,
            &bundle.bytes,
            Some(&bad_resource),
            persona,
            None
        ),
        Err(Error::Authentication)
    ))?;
    check(
        version[5] == 2
            && matches!(
                restore(&kit.secret, &bundle.bytes[..20], None, persona, None),
                Err(Error::Encoding)
            ),
    )
}

fn replay_forgery() -> Result<()> {
    let (_, _, owner, _, mut h, _) = fixture();
    let target = key();
    let device = random_id();
    let base = h.clone();
    grant(&mut h, &owner, device, CONTROLLER_SCOPE, &target)?;
    let event = h
        .events
        .values()
        .find(|e| matches!(e.action, Action::Grant { .. }))
        .unwrap()
        .clone();
    check(h.insert(event.clone()) == Err(Error::Replay))?;
    let mut forged = event;
    forged.signature[0] ^= 1;
    forged.command = random_id();
    check(h.insert(forged) == Err(Error::Authentication))?;
    let mut bad = h.clone();
    bad.events
        .values_mut()
        .find(|e| matches!(e.action, Action::Grant { .. }))
        .unwrap()
        .signature[0] ^= 1;
    let mut merged = base.clone();
    check(
        merged.merge(&bad) == Err(Error::Authentication)
            && merged.events.len() == base.events.len(),
    )
}

fn atomic_command_retry() -> Result<()> {
    let (_, _, owner, _, base, _) = fixture();
    let mut proposed = base.clone();
    let id = proposed.append(
        &owner,
        random_id(),
        Action::Revoke {
            device: random_id(),
            scope: random_id(),
        },
    )?;
    let event = proposed.events.get(&id).unwrap().clone();
    let mut store = StateStore::new(base)?;
    for cut in [
        CrashCut::BeforeWrite,
        CrashCut::AfterWrite,
        CrashCut::AfterReadback,
        CrashCut::BeforePromote,
    ] {
        check(store.commit(event.clone(), cut) == Err(Error::Crash))?;
        store.reopen();
        check(!store.history().events.contains_key(&id))?;
    }
    check(store.commit(event.clone(), CrashCut::None)? == id)?;
    check(store.commit(event, CrashCut::None)? == id && store.history().events.contains_key(&id))
}

fn concurrent_restores() -> Result<()> {
    let (persona, _, _, kit, h, _) = fixture();
    let bundle = kit.export_bundle(&h, vec![])?;
    let a = restore(&kit.secret, &bundle.bytes, None, persona, None)?;
    let b = restore(&kit.secret, &bundle.bytes, None, persona, None)?;
    let mut merged = a.history;
    merged.merge(&b.history)?;
    check(
        merged.events.len() == h.events.len() + 2
            && matches!(
                merged.access(a.device, CONTROLLER_SCOPE),
                Access::Provisional(_)
            ),
    )?;
    check(matches!(
        merged.access(b.device, CONTROLLER_SCOPE),
        Access::Provisional(_)
    ))
}

fn rotation() -> Result<()> {
    let (persona, _, owner, kit, mut h, _) = fixture();
    let old = kit.export_bundle(&h, vec![])?;
    let mut prepared = kit.prepare_rotation(&h, vec![])?;
    check(restore(&kit.secret, &old.bytes, None, persona, Some(&h)).is_ok())?;
    check(matches!(
        restore(
            &prepared.next.secret,
            &prepared.export.bytes,
            None,
            persona,
            None
        ),
        Err(Error::Stale)
    ))?;
    let readback = prepared.export.bytes.clone();
    check(prepared.commit(&mut h, &owner) == Err(Error::Verification))?;
    prepared.verify_export(&readback)?;
    check(prepared.commit(&mut h, &owner) == Err(Error::Verification))?;
    prepared.confirm_custody();
    prepared.export.bytes[0] ^= 1;
    check(prepared.commit(&mut h, &owner) == Err(Error::Verification))?;
    prepared.export.bytes = readback;
    prepared.commit(&mut h, &owner)?;
    check(matches!(
        restore(&kit.secret, &old.bytes, None, persona, Some(&h)),
        Err(Error::Stale)
    ))?;
    let stale = restore(&kit.secret, &old.bytes, None, persona, None)?;
    check(stale.authority == Authority::Provisional)?;
    check(
        restore(
            &prepared.next.secret,
            &prepared.export.bytes,
            None,
            persona,
            Some(&h),
        )
        .is_ok(),
    )
}

fn conflicting_grant_revoke() -> Result<()> {
    let (_, _, owner, _, mut base, _) = fixture();
    let c = key();
    let c_id = random_id();
    let scope = random_id();
    grant(&mut base, &owner, c_id, scope, &c)?;
    let signer_a = key();
    let signer_b = key();
    grant(&mut base, &owner, random_id(), CONTROLLER_SCOPE, &signer_a)?;
    grant(&mut base, &owner, random_id(), CONTROLLER_SCOPE, &signer_b)?;
    let mut a = base.clone();
    let mut b = base;
    grant(&mut a, &signer_a, c_id, scope, &c)?;
    b.append(
        &signer_b,
        random_id(),
        Action::Revoke {
            device: c_id,
            scope,
        },
    )?;
    let branches = [
        *a.frontier().first().unwrap(),
        *b.frontier().first().unwrap(),
    ];
    a.merge(&b)?;
    check(a.access(c_id, scope) == Access::Conflict)?;
    let mut sorted = branches;
    sorted.sort();
    check(matches!(
        a.append(
            &owner,
            random_id(),
            Action::Resolve {
                device: c_id,
                scope,
                branches: sorted,
                key: Some(key().verifying_key().to_bytes()),
            }
        ),
        Err(Error::Authority)
    ))?;
    a.append(
        &owner,
        random_id(),
        Action::Resolve {
            device: c_id,
            scope,
            branches: sorted,
            key: None,
        },
    )?;
    check(a.access(c_id, scope) == Access::Denied)?;
    check(matches!(
        a.append(
            &owner,
            random_id(),
            Action::Resolve {
                device: c_id,
                scope,
                branches: sorted,
                key: None,
            }
        ),
        Err(Error::Authority)
    ))
}

fn concurrent_same_intent() -> Result<()> {
    let (_, _, owner, _, mut base, _) = fixture();
    let approver_a = key();
    let approver_b = key();
    grant(
        &mut base,
        &owner,
        random_id(),
        CONTROLLER_SCOPE,
        &approver_a,
    )?;
    grant(
        &mut base,
        &owner,
        random_id(),
        CONTROLLER_SCOPE,
        &approver_b,
    )?;
    let device = random_id();
    let target = key();
    let intent = EnrollmentIntent::new(base.persona, device, CONTROLLER_SCOPE, 100, &target);
    let mut a = base.clone();
    let mut b = base;
    a.append(
        &approver_a,
        random_id(),
        Action::Grant {
            intent: intent.clone(),
            approved_at: 1,
        },
    )?;
    b.append(
        &approver_b,
        random_id(),
        Action::Grant {
            intent,
            approved_at: 1,
        },
    )?;
    a.merge(&b)?;
    check(a.access(device, CONTROLLER_SCOPE) == Access::Conflict)?;
    let mut branches: [vida_core::Digest; 2] =
        a.frontier().try_into().map_err(|_| Error::Verification)?;
    branches.sort();
    a.append(
        &owner,
        random_id(),
        Action::Resolve {
            device,
            scope: CONTROLLER_SCOPE,
            branches,
            key: Some(target.verifying_key().to_bytes()),
        },
    )?;
    check(a.access(device, CONTROLLER_SCOPE) == Access::Verified(target.verifying_key().to_bytes()))
}

fn independent_grants() -> Result<()> {
    let (_, _, owner, _, mut base, _) = fixture();
    let a_key = key();
    let b_key = key();
    let a_id = random_id();
    let b_id = random_id();
    let signer_a = key();
    let signer_b = key();
    grant(&mut base, &owner, random_id(), CONTROLLER_SCOPE, &signer_a)?;
    grant(&mut base, &owner, random_id(), CONTROLLER_SCOPE, &signer_b)?;
    let mut a = base.clone();
    let mut b = base;
    grant(&mut a, &signer_a, a_id, CONTROLLER_SCOPE, &a_key)?;
    grant(&mut b, &signer_b, b_id, CONTROLLER_SCOPE, &b_key)?;
    a.merge(&b)?;
    check(a.access(a_id, CONTROLLER_SCOPE) == Access::Verified(a_key.verifying_key().to_bytes()))?;
    check(a.access(b_id, CONTROLLER_SCOPE) == Access::Verified(b_key.verifying_key().to_bytes()))
}

fn reused_device_key() -> Result<()> {
    let (_, _, owner, _, mut base, _) = fixture();
    let shared = key();
    let first = random_id();
    let second = random_id();
    grant(&mut base, &owner, first, CONTROLLER_SCOPE, &shared)?;
    check(matches!(
        grant(&mut base, &owner, second, CONTROLLER_SCOPE, &shared),
        Err(Error::Authority)
    ))?;
    let (_, _, owner, _, mut base, _) = fixture();
    let approver_a = key();
    let approver_b = key();
    grant(
        &mut base,
        &owner,
        random_id(),
        CONTROLLER_SCOPE,
        &approver_a,
    )?;
    grant(
        &mut base,
        &owner,
        random_id(),
        CONTROLLER_SCOPE,
        &approver_b,
    )?;
    let mut a = base.clone();
    let mut b = base;
    grant(&mut a, &approver_a, second, CONTROLLER_SCOPE, &shared)?;
    let third = random_id();
    grant(&mut b, &approver_b, third, CONTROLLER_SCOPE, &shared)?;
    a.merge(&b)?;
    check(a.access(second, CONTROLLER_SCOPE) == Access::Conflict)?;
    check(a.access(third, CONTROLLER_SCOPE) == Access::Conflict)?;
    check(matches!(
        a.append(
            &shared,
            random_id(),
            Action::Revoke {
                device: third,
                scope: CONTROLLER_SCOPE
            }
        ),
        Err(Error::Authority)
    ))
}

fn recovery_id_collision() -> Result<()> {
    let persona = random_id();
    let device = random_id();
    let owner = key();
    let recovery = key();
    let mut history = History::new(persona, device, &owner, recovery.verifying_key().to_bytes());
    check(matches!(
        history.append(
            &recovery,
            random_id(),
            Action::RecoveryGrant {
                device,
                key: key().verifying_key().to_bytes(),
            }
        ),
        Err(Error::Authority)
    ))
}

fn unrelated_known_history() -> Result<()> {
    let (persona, _, owner, kit, history, _) = fixture();
    let bundle = kit.export_bundle(&history, vec![])?;
    let unrelated = History::new(persona, random_id(), &owner, kit.recovery_key());
    check(matches!(
        restore(&kit.secret, &bundle.bytes, None, persona, Some(&unrelated)),
        Err(Error::Stale)
    ))
}

fn single_note_manifest() -> Result<()> {
    let (_, _, _, kit, history, note) = fixture();
    let mut coverage = note_coverage(&note);
    coverage.push(Coverage {
        id: random_id(),
        frontier: [0x44; 32],
    });
    check(matches!(
        kit.export_bundle(&history, coverage),
        Err(Error::Encoding)
    ))
}

fn backup_frontier() -> Result<()> {
    let (persona, _, _, kit, h, note) = fixture();
    let export = kit.export_note(persona, &h, &note)?;
    let later = Note {
        id: note.id,
        content: b"later local edit".to_vec(),
        frontier: [0x67; 32],
    };
    check(export.frontier != later.frontier)?;
    let bundle = kit.export_bundle(&h, note_coverage(&note))?;
    let later_bytes = kit.export_note(persona, &h, &later)?;
    check(matches!(
        restore(
            &kit.secret,
            &bundle.bytes,
            Some(&later_bytes.bytes),
            persona,
            None
        ),
        Err(Error::MissingResource)
    ))?;
    let restored = restore(
        &kit.secret,
        &bundle.bytes,
        Some(&export.bytes),
        persona,
        None,
    )?;
    check(
        restored.note.as_ref() == Some(&note)
            && restored.note.as_ref() != Some(&later)
            && restored.content_frontier == Some(export.frontier),
    )
}

fn readback() -> Result<()> {
    let (persona, _, _, kit, h, note) = fixture();
    let bundle = kit.export_bundle(&h, note_coverage(&note))?;
    let encrypted = kit.export_note(persona, &h, &note)?;
    let oracle = |bytes: &[u8]| {
        let restored = restore(&kit.secret, &bundle.bytes, Some(bytes), persona, None)?;
        if restored.note.as_ref() != Some(&note) || restored.content_frontier != Some(note.frontier)
        {
            return Err(Error::Verification);
        }
        Ok((note.frontier, vec![note.id]))
    };
    let mut s = MemoryStore::default();
    s.export(
        "resource",
        encrypted.bytes.clone(),
        CrashCut::None,
        true,
        oracle,
    )?;
    check(s.current("resource").unwrap().frontier == note.frontier)?;
    check(matches!(
        s.export(
            "resource",
            encrypted.bytes.clone(),
            CrashCut::AfterReadback,
            true,
            oracle
        ),
        Err(Error::Crash)
    ))?;
    check(s.current("resource").unwrap().bytes == encrypted.bytes)?;
    let mut tampered = encrypted.bytes.clone();
    *tampered.last_mut().unwrap() ^= 1;
    check(matches!(
        s.export("resource", tampered, CrashCut::None, true, oracle),
        Err(Error::Authentication)
    ))?;
    check(s.current("resource").unwrap().bytes == encrypted.bytes)?;
    check(matches!(
        s.export(
            "resource",
            encrypted.bytes.clone(),
            CrashCut::None,
            true,
            |_| Err(Error::Verification)
        ),
        Err(Error::Verification)
    ))?;
    check(s.current("resource").unwrap().bytes == encrypted.bytes)?;
    check(!s.export(
        "resource",
        encrypted.bytes.clone(),
        CrashCut::None,
        false,
        |_| panic!("no readback must not call oracle"),
    )?)?;
    check(s.current("resource").unwrap().bytes == encrypted.bytes)
}

fn stale_rotation() -> Result<()> {
    let (_, _, owner, kit, mut h, note) = fixture();
    let mut prepared = kit.prepare_rotation(&h, note_coverage(&note))?;
    let readback = prepared.export.bytes.clone();
    prepared.verify_export(&readback)?;
    prepared.confirm_custody();
    h.append(
        &owner,
        random_id(),
        Action::Revoke {
            device: random_id(),
            scope: random_id(),
        },
    )?;
    check(prepared.commit(&mut h, &owner) == Err(Error::Stale))
}

fn retained_snapshot() -> Result<()> {
    let (_, _, _, _, h, note) = fixture();
    let f = h.frontier_hash();
    let mut s = MemoryStore::default();
    s.export("resource", vec![1], CrashCut::None, true, |_| {
        Ok((f, vec![note.id]))
    })?;
    s.export("resource", vec![2], CrashCut::None, true, |_| {
        Ok((f, vec![note.id]))
    })?;
    check(s.previous("resource").len() == 1 && s.previous("resource")[0].bytes == vec![1])
}

fn scoped_grants() -> Result<()> {
    let (_, _, owner, _, mut h, _) = fixture();
    let d = random_id();
    let k = key();
    let personal = random_id();
    let work = random_id();
    grant(&mut h, &owner, d, personal, &k)?;
    grant(&mut h, &owner, d, work, &k)?;
    h.append(
        &owner,
        random_id(),
        Action::Revoke {
            device: d,
            scope: personal,
        },
    )?;
    check(
        h.access(d, personal) == Access::Denied
            && h.access(d, work) == Access::Verified(k.verifying_key().to_bytes()),
    )
}

#[derive(Serialize)]
struct Case {
    id: String,
    status: &'static str,
    reason: String,
}
type Fixture = (&'static str, fn() -> Result<()>);

#[test]
fn conformance_matrix() {
    let proven: [Fixture; 14] = [
        ("REC-F02", enrollment),
        ("REC-F03", fresh_restore),
        ("REC-F04", missing_parts),
        ("REC-F05", wrong_secret),
        ("REC-F06", tamper_version),
        ("REC-F07", replay_forgery),
        ("REC-F13", concurrent_restores),
        ("REC-F15", rotation),
        ("REC-F17", conflicting_grant_revoke),
        ("REC-F19", independent_grants),
        ("REC-F22", backup_frontier),
        ("REC-F23", readback),
        ("REC-F25", stale_rotation),
        ("REC-F29", scoped_grants),
    ];
    let mut cases = Vec::new();
    for n in 1..=29 {
        if n == 9 {
            cases.push(Case {
                id: "REC-F09a".into(),
                status: "unproven",
                reason: "compromise cut and platform custody need integration proof".into(),
            });
        }
        let id = format!("REC-F{n:02}");
        if let Some((_, run)) = proven.iter().find(|(name, _)| *name == id) {
            let result = std::panic::catch_unwind(*run);
            let (status, reason) = match result {
                Ok(Ok(())) => ("passed", "headless Rust fixture passed".into()),
                Ok(Err(e)) => ("failed", e.to_string()),
                Err(_) => ("failed", "fixture panicked".into()),
            };
            cases.push(Case { id, status, reason });
        } else {
            let reason = match n {
                1 | 8 => "persistent bootstrap and restart adapter absent",
                9 | 14 | 16 | 18 => "distributed authority and compromise proof absent",
                10 | 11 | 12 | 20 | 21 | 27 => "platform, account, UI or telemetry adapter absent",
                24 => "fresh-profile release gate and repair UI absent",
                26 => "headless critical-risk projection passes; UI and release-gate integration absent",
                28 => "checked separately by retained_snapshot; off-device retention gate absent",
                _ => "platform and end-to-end evidence absent",
            };
            cases.push(Case {
                id,
                status: "unproven",
                reason: reason.into(),
            });
        }
    }
    let dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target");
    fs::create_dir_all(&dir).unwrap();
    fs::write(
        dir.join("rec-case-matrix.json"),
        serde_json::to_vec_pretty(&cases).unwrap(),
    )
    .unwrap();
    assert!(
        cases.iter().all(|c| c.status != "failed"),
        "conformance fixture failed; see target/rec-case-matrix.json"
    );
}

#[test]
fn headless_model_guards() {
    atomic_command_retry().unwrap();
    concurrent_same_intent().unwrap();
    reused_device_key().unwrap();
    recovery_id_collision().unwrap();
    unrelated_known_history().unwrap();
    single_note_manifest().unwrap();
}

#[test]
fn verified_snapshots_remain_available() {
    retained_snapshot().unwrap();
}

#[test]
fn trusted_repair_does_not_reactivate_old_kit() {
    let persona = random_id();
    let owner = key();
    let data_key = [0x55; 32];
    let kit = Kit::new(Some(data_key));
    let mut history = History::new(persona, random_id(), &owner, kit.recovery_key());
    let old = kit.export_bundle(&history, vec![]).unwrap();
    let mut repair = Kit::repair_from_trusted(data_key, &history, vec![]).unwrap();
    let readback = repair.export.bytes.clone();
    repair.verify_export(&readback).unwrap();
    repair.confirm_custody();
    repair.commit(&mut history, &owner).unwrap();
    assert!(matches!(
        restore(&kit.secret, &old.bytes, None, persona, Some(&history)),
        Err(Error::Stale)
    ));
    assert!(restore(
        &repair.next.secret,
        &repair.export.bytes,
        None,
        persona,
        Some(&history)
    )
    .is_ok());
}

#[test]
fn failed_post_commit_restore_is_critical() {
    let (persona, _, owner, kit, mut history, note) = fixture();
    let resource = kit.export_note(persona, &history, &note).unwrap();
    let mut rotation = kit
        .prepare_rotation(&history, note_coverage(&note))
        .unwrap();
    assert!(matches!(
        RotationHealth::after_commit(&history, &rotation.export.bytes),
        Err(Error::Stale)
    ));
    let readback = rotation.export.bytes.clone();
    rotation.verify_export(&readback).unwrap();
    rotation.confirm_custody();
    rotation.commit(&mut history, &owner).unwrap();
    let mut successful = RotationHealth::after_commit(&history, &rotation.export.bytes).unwrap();
    assert!(successful
        .test_restore(
            &rotation.next.secret,
            &rotation.export.bytes,
            Some(&resource.bytes)
        )
        .is_ok());
    assert_eq!(successful.status, RotationProofStatus::Passed);
    let mut health = RotationHealth::after_commit(&history, &rotation.export.bytes).unwrap();
    assert_eq!(health.status, RotationProofStatus::Pending);
    let mut damaged = resource.bytes.clone();
    *damaged.last_mut().unwrap() ^= 1;
    assert!(matches!(
        health.test_restore(
            &rotation.next.secret,
            &rotation.export.bytes,
            Some(&damaged)
        ),
        Err(Error::Authentication)
    ));
    assert_eq!(health.status, RotationProofStatus::Critical);
    assert!(health
        .test_restore(
            &rotation.next.secret,
            &rotation.export.bytes,
            Some(&resource.bytes)
        )
        .is_ok());
    assert_eq!(health.status, RotationProofStatus::Critical);
}

#[test]
fn golden_event_v1() {
    use ed25519_dalek::Signer;
    let signing = SigningKey::from_bytes(&[7; 32]);
    let mut event = Event {
        persona: [0x11; 16],
        signer: signing.verifying_key().to_bytes(),
        sequence: 0,
        parents: vec![],
        epoch: 0,
        command: [0x22; 16],
        action: Action::Genesis {
            device: [0x33; 16],
            key: signing.verifying_key().to_bytes(),
            recovery_key: [0x44; 32],
        },
        signature: vec![0; 64],
    };
    event.signature = signing
        .sign(&event.signed_bytes().unwrap())
        .to_bytes()
        .to_vec();
    let signed_hex: String = event
        .signed_bytes()
        .unwrap()
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect();
    let id_hex: String = event
        .id()
        .unwrap()
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect();
    assert_eq!(signed_hex, "564944412d4354524c2d4556454e542d76310011111111111111111111111111111111ea4a6c63e29c520abef5507b132ec5f9954776aebebe7b92421eea691446d22c0000000000000000000000000000000000222222222222222222222222222222220033333333333333333333333333333333ea4a6c63e29c520abef5507b132ec5f9954776aebebe7b92421eea691446d22c4444444444444444444444444444444444444444444444444444444444444444");
    assert_eq!(
        id_hex,
        "3a8c3dade6b173e01da2a688221dcebacea1e2f7464b0cf86c6ce8ac277c56e3"
    );
    event.verify().unwrap();
}
