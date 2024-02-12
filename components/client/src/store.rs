use super::error::Error;
use common::{
    keys::AsymmetricKeyPair,
    log::{info, Logger},
};
use reqwest;

/// Store key on the server.
pub fn store_key_pair(
    log: &Logger,
    host_url: &str,
    access_token: &str,
    user_id: &str,
    key: &AsymmetricKeyPair,
) -> Result<(), Error> {
    // TODO: Maybe: Redirect stdout to stderr and output json on stdout.

    let client = reqwest::blocking::Client::new();

    let req = client
        .put(format!("{}/api/v1/{}/keys", host_url, user_id))
        .json(&key)
        .build()?;

    let res = client.execute(req)?;
    let key = res.json();
    info!(log, "Succesfully stored on server with key id: '{}'", ));

    Ok(())
}
