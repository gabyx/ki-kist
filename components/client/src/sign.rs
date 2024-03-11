use crate::password::get_passphrase;

use super::error::Error;
use common::{
    keys::AsymmetricKeyPair,
    log::{info, Logger},
    path::extension::add_extension,
};
use libsignify::{Codeable, PrivateKey};
use snafu::whatever;
use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::{Path, PathBuf},
};

/// Sign a file with an asymmetric key pair.
pub fn sign_file(
    log: &Logger,
    non_interactive: bool,
    key_pair: &AsymmetricKeyPair,
    passphrase_file: &Option<PathBuf>,
    file: &Path,
    file_signature: Option<PathBuf>,
) -> Result<(), Error> {
    let passphrase = get_passphrase(non_interactive, passphrase_file)?;

    let mut secret_key: PrivateKey =
        <PrivateKey as Codeable>::from_base64(&key_pair.private_key_encrypted)?
            .0;

    if !secret_key.is_encrypted() {
        whatever!("The private secrete key is not encrypted, which is wrong.")
    }

    info!(log, "Decrypt private key ...");
    secret_key.decrypt_with_password(&passphrase)?;

    let mut msg_file = File::open(file)?;
    let mut msg = vec![];
    msg_file.read_to_end(&mut msg)?;

    let file_signature = match file_signature {
        Some(path) => path,
        None => add_extension(file, "sig"),
    };

    info!(log, "Signing file '{}'...", file.display());
    let sig = secret_key.sign(&msg);
    let sig_comment = "ki-kist signature from secret key (signify)";

    let mut f = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(file_signature)?;

    f.write_all(&sig.to_file_encoding(sig_comment))?;

    info!(
        log,
        "Successfully created signature for file: '{}'.",
        file.display()
    );

    return Ok(());
}
