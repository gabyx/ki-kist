use super::error::Error;
use common::{
    keys::AsymmetricKeyPair,
    log::{info, Logger},
};
use libsignify::{Codeable, NewKeyOpts, PrivateKey, PublicKey, Signature};
use rpassword;
use snafu::whatever;
use std::{
    io::Read,
    path::{Path, PathBuf},
};

/// Store key on the server.
pub fn store_key_pair(
    _log: &Logger,
    _key: &AsymmetricKeyPair,
    _host_url: String,
    _access_token: String,
) -> Result<(), Error> {
    Ok(())
}
