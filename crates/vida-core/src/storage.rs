use crate::controller::{Event, History};
use crate::{Digest, Error, Result};
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CrashCut {
    None,
    BeforeWrite,
    AfterWrite,
    AfterReadback,
    BeforePromote,
}

#[derive(Clone, Debug)]
pub struct StateStore {
    committed: History,
    pending: Option<History>,
}

impl StateStore {
    pub fn new(history: History) -> Result<Self> {
        history.validate()?;
        Ok(Self {
            committed: history,
            pending: None,
        })
    }

    pub fn history(&self) -> &History {
        &self.committed
    }

    pub fn reopen(&mut self) {
        self.pending = None;
    }

    pub fn commit(&mut self, event: Event, cut: CrashCut) -> Result<Digest> {
        if let Some((id, existing)) = self
            .committed
            .events
            .iter()
            .find(|(_, existing)| existing.command == event.command)
        {
            return if existing == &event {
                Ok(*id)
            } else {
                Err(Error::Replay)
            };
        }
        if cut == CrashCut::BeforeWrite {
            return Err(Error::Crash);
        }
        let mut candidate = self.committed.clone();
        let id = candidate.insert(event)?;
        self.pending = Some(candidate);
        if cut == CrashCut::AfterWrite {
            return Err(Error::Crash);
        }
        self.pending
            .as_ref()
            .ok_or(Error::Verification)?
            .validate()?;
        if matches!(cut, CrashCut::AfterReadback | CrashCut::BeforePromote) {
            return Err(Error::Crash);
        }
        self.committed = self.pending.take().ok_or(Error::Verification)?;
        Ok(id)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VerifiedSnapshot {
    pub frontier: Digest,
    pub coverage: Vec<[u8; 16]>,
    pub bytes: Vec<u8>,
    pub independent_destination: bool,
}

#[derive(Clone, Debug, Default)]
pub struct MemoryStore {
    active: BTreeMap<String, VerifiedSnapshot>,
    previous: BTreeMap<String, Vec<VerifiedSnapshot>>,
    pending: BTreeMap<String, Vec<u8>>,
}

impl MemoryStore {
    pub fn current(&self, class: &str) -> Option<&VerifiedSnapshot> {
        self.active.get(class)
    }
    pub fn previous(&self, class: &str) -> &[VerifiedSnapshot] {
        self.previous.get(class).map_or(&[], Vec::as_slice)
    }
    pub fn pending(&self, class: &str) -> Option<&[u8]> {
        self.pending.get(class).map(Vec::as_slice)
    }

    pub fn export<F>(
        &mut self,
        class: &str,
        bytes: Vec<u8>,
        cut: CrashCut,
        readback: bool,
        oracle: F,
    ) -> Result<bool>
    where
        F: Fn(&[u8]) -> Result<(Digest, Vec<[u8; 16]>)>,
    {
        if cut == CrashCut::BeforeWrite {
            return Err(Error::Crash);
        }
        self.pending.insert(class.to_owned(), bytes);
        if cut == CrashCut::AfterWrite {
            return Err(Error::Crash);
        }
        if !readback {
            return Ok(false);
        }
        let copy = self.pending.get(class).ok_or(Error::Verification)?.clone();
        let (frontier, coverage) = oracle(&copy)?;
        if cut == CrashCut::AfterReadback || cut == CrashCut::BeforePromote {
            return Err(Error::Crash);
        }
        let snapshot = VerifiedSnapshot {
            frontier,
            coverage,
            bytes: copy,
            independent_destination: false,
        };
        if let Some(prior) = self.active.insert(class.to_owned(), snapshot) {
            self.previous
                .entry(class.to_owned())
                .or_default()
                .push(prior);
        }
        self.pending.remove(class);
        Ok(true)
    }

    pub fn mark_independent(&mut self, class: &str, independently_readable: bool) -> Result<()> {
        let snapshot = self.active.get_mut(class).ok_or(Error::MissingResource)?;
        snapshot.independent_destination = independently_readable;
        Ok(())
    }
}
