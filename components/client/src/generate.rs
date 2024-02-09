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

fn read_passphrase(confirm: bool) -> Result<String, Error> {
    let pass = rpassword::prompt_password("Enter passphrase: ")?;

    if confirm {
        let conf_pass = rpassword::prompt_password("Confirm passphrase: ")?;

        if pass != conf_pass {
            whatever!("Passwords don't match");
        }
    }

    Ok(pass)
}

fn read_passphrase_file(passphrase_file: &Path) -> Result<String, Error> {
    let mut f = std::fs::OpenOptions::new()
        .read(true)
        .open(passphrase_file)?;

    let mut passphrase = String::new();
    f.read_to_string(&mut passphrase)?;

    passphrase = passphrase.lines().take(1).collect::<String>();

    return Ok(passphrase);
}

pub fn validate_passphrase(passphrase: &str) -> Result<(), Error> {
    let analysis = passwords::analyzer::analyze(passphrase);
    let score = passwords::scorer::score(&analysis);

    if score < 80.0 {
        whatever!(
            "Your passphrase score '{}' must be >= 80% to be considered safe.",
            score
        );
    }

    return Ok(());
}

pub fn generate_asymmetric_key_pair(
    log: &Logger,
    non_interactive: bool,
    passphrase_file: &Option<PathBuf>,
) -> Result<AsymmetricKeyPair, Error> {
    let passphrase: String;

    if non_interactive || passphrase_file.is_some() {
        if non_interactive && !passphrase_file.is_some() {
            whatever!("You need to specify password file in non-interactive mode.");
        }
        passphrase = read_passphrase_file(passphrase_file.as_ref().unwrap())?;
    } else {
        passphrase = read_passphrase(true)?;
    }

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
        private_key_encrypted: String::from_utf8(
            private_key.to_file_encoding("kikist generated private encrypted key (signify)."),
        )
        .expect("Utf8 encoding error."),

        public_key: String::from_utf8(
            private_key
                .public()
                .to_file_encoding("kikist generated public key (signify)."),
        )
        .expect("Utf8 encoding error."),
    };

    info!(
        log,
        "Public Key File 'key.pub' content:\n{}", key.public_key
    );

    info!(
        log,
        "Private Key File 'key.prv' content:\n{}", key.private_key_encrypted
    );

    return Ok(key);
}
