pub mod controller;
pub mod recovery;
pub mod storage;

use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum Error {
    #[error("invalid encoding or unsupported mandatory version")]
    Encoding,
    #[error("authentication failed")]
    Authentication,
    #[error("causal parent or signer authority missing")]
    Authority,
    #[error("replayed command or event")]
    Replay,
    #[error("stale frontier or epoch")]
    Stale,
    #[error("resource copy missing or incomplete")]
    MissingResource,
    #[error("readback or validation failed")]
    Verification,
    #[error("injected crash cut")]
    Crash,
}

pub type Result<T> = std::result::Result<T, Error>;
pub type Id = [u8; 16];
pub type Digest = [u8; 32];

pub fn random_id() -> Id {
    use rand_core::RngCore;
    let mut id = [0; 16];
    rand_core::OsRng.fill_bytes(&mut id);
    id
}

pub(crate) fn digest(bytes: &[u8]) -> Digest {
    use sha2::Digest as _;
    sha2::Sha256::digest(bytes).into()
}
