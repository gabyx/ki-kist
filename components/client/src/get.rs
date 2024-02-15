use super::error::Error;
use common::{keys::AsymmetricKeyPair, log::Logger, messages::GetKeyResponse};
use uuid::Uuid;

/// Store key on the server.
pub fn get_key_pair(
    log: &Logger,
    host_url: &str,
    _access_token: &str,
    user_id: &str,
    key_id: &Uuid,
) -> Result<AsymmetricKeyPair, Error> {
    // TODO: Maybe: Redirect stdout to stderr and output json on stdout.
    // TODO: Improve request error handling accroding to spec.

    let client = reqwest::blocking::Client::new();
    let res = client
        .get(format!("{}/api/v1/{}/keys/{}", host_url, user_id, key_id))
        .send()?;

    let key_pair: &AsymmetricKeyPair = &res.json::<GetKeyResponse>()?.0;

    key_pair.log(log);

    return Ok(key_pair.clone());
}
