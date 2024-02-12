use crate::keys::AsymmetricKeyPair;
use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct StoreKeyRequest(pub AsymmetricKeyPair);

#[derive(Debug, Serialize, Deserialize)]
pub struct StoreKeyResponse {
    pub key_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetKeyResponse(pub AsymmetricKeyPair);
