use super::error::Error;
use snafu::whatever;
use std::{
    io::Read,
    path::{Path, PathBuf},
};

/// Get a passphrase either from stdin or from the file.
pub fn get_passphrase(
    non_interactive: bool,
    passphrase_file: &Option<PathBuf>,
) -> Result<String, Error> {
    let passphrase = {
        if non_interactive || passphrase_file.is_some() {
            if non_interactive && !passphrase_file.is_some() {
                whatever!(
                "You need to specify password file in non-interactive mode."
            );
            }
            read_passphrase_file(passphrase_file.as_ref().unwrap())?
        } else {
            read_passphrase(true)?
        }
    };

    return Ok(passphrase);
}

/// Read passphrase from stdin.
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

/// Read passphrase from file.
fn read_passphrase_file(passphrase_file: &Path) -> Result<String, Error> {
    let mut f = std::fs::OpenOptions::new()
        .read(true)
        .open(passphrase_file)?;

    let mut passphrase = String::new();
    f.read_to_string(&mut passphrase)?;

    passphrase = passphrase.lines().take(1).collect::<String>();

    return Ok(passphrase);
}

/// Validate a passphrase.
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
