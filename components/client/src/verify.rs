use super::error::Error;
use common::{
    keys::AsymmetricKeyPair,
    log::{info, Logger},
    path::extension::add_extension,
};
use libsignify::{Codeable, PublicKey, Signature};
use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

/// Sign a file with an asymmetric key pair.
pub fn verify_file(
    log: &Logger,
    key_pair: &AsymmetricKeyPair,
    file: &Path,
    file_signature: Option<PathBuf>,
) -> Result<(), Error> {
    let public_key: PublicKey =
        <PublicKey as Codeable>::from_base64(&key_pair.public_key)?.0;

    info!(log, "Read signature file ...");

    let file_signature = match file_signature {
        Some(path) => path,
        None => add_extension(file, "sig"),
    };

    let mut sig_file = File::open(file_signature)?;
    let mut sig = String::new();
    sig_file.read_to_string(&mut sig)?;
    let sig: Signature = <_ as Codeable>::from_base64(&sig)?.0;

    let mut msg = vec![];
    let mut f = File::open(file)?;
    f.read_to_end(&mut msg)?;

    public_key.verify(&msg, &sig)?;

    info!(
        log,
        "Successfully verified signature for file: '{}'.",
        file.display()
    );

    return Ok(());
}
