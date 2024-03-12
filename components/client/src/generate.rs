use crate::password::validate_passphrase;

use super::{error::Error, password::get_passphrase};
use common::{
    keys::AsymmetricKeyPair,
    log::{info, Logger},
};
use libsignify::{Codeable, NewKeyOpts, PrivateKey};
use snafu::whatever;
use std::{path::PathBuf, thread::sleep, time::Duration};

/// KDF stands for Key Derivation Function, and it is a cryptographic function used
/// to derive one or more secret keys from a given input, typically a password or
/// passphrase. KDF iterations refer to the number of times the underlying
/// cryptographic algorithm of the KDF is applied to the input data. The purpose of
/// using multiple iterations is to increase the computational effort required for
/// attackers attempting to perform brute-force or dictionary attacks.
/// By iterating the KDF multiple times, the process becomes more time-consuming
/// and resource-intensive, making it harder for attackers to guess the original
/// input (e.g., a password) through trial and error. This technique is commonly
/// employed to enhance the security of stored passwords or other sensitive
/// cryptographic keys.
/// We use here a fixed constant, bigger then the OpenBSD signify thing:
const KDF_ROUNDS: u32 = 50;

pub fn generate_asymmetric_key_pair(
    log: &Logger,
    non_interactive: bool,
    passphrase_file: &Option<PathBuf>,
) -> Result<AsymmetricKeyPair, Error> {
    info!(log, "Generate a new asymmetric key pair.");

    let passphrase = get_passphrase(non_interactive, passphrase_file)?;

    info!(log, "Validate password ...");
    validate_passphrase(&passphrase)?;

    // Store the private key
    let mut rng = rand_core::OsRng {};
    let res = PrivateKey::generate(
        &mut rng,
        NewKeyOpts::Encrypted {
            passphrase,
            kdf_rounds: KDF_ROUNDS,
        },
    );

    let private_key = whatever!(res, "Key generation failed");

    let key = AsymmetricKeyPair {
        private_key_encrypted: String::from_utf8(private_key.to_file_encoding(
            "kikist generated private encrypted key (signify).",
        ))
        .expect("Utf8 encoding error."),

        public_key: String::from_utf8(
            private_key
                .public()
                .to_file_encoding("kikist generated public key (signify)."),
        )
        .expect("Utf8 encoding error."),
    };

    key.log(log);

    return Ok(key);
}
