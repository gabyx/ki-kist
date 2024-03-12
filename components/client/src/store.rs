use super::error::Error;
use common::{
    keys::AsymmetricKeyPair,
    log::{info, Logger},
    messages::StoreKeyResponse,
};

/// Store key on the server.
pub fn store_key_pair(
    log: &Logger,
    host_url: &str,
    _access_token: &str,
    user_id: &str,
    key: &AsymmetricKeyPair,
) -> Result<(), Error> {
    // TODO: Maybe: Redirect stdout to stderr and output json on stdout.
    // TODO: Improve request error handling accroding to spec.
    //
    let client = reqwest::blocking::Client::new();

    info!(log, "Store key pair for user {user_id}");
    let res = client
        .put(format!("{}/api/v1/{}/keys", host_url, user_id))
        .json(&key)
        .send()?;

    let res = res.json::<StoreKeyResponse>()?;

    info!(
        log,
        "Succesfully stored on server with key id: '{}'", res.key_id
    );

    Ok(())
}
