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
    let client = reqwest::blocking::Client::new();

    // let k = AsymmetricKeyPair {
    //     public_key: "asdf".to_owned(),
    //     private_key_encrypted: "asdf".to_owned(),
    // };
    let req = client
        .put(format!("{}/api/v1/{}/keys", host_url, user_id))
        .json(&key)
        .build()?;

    info!(
        log,
        "{}",
        String::from_utf8_lossy(req.body().unwrap().as_bytes().unwrap())
    );
    info!(log, "Request: {:?}", req.body());

    let res = client.execute(req)?;

    info!(log, "Request {:?}", res);

    Ok(())
}
