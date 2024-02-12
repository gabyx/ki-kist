use std::ops::Deref;

use crate::keys::AsymmetricKeyPair;
use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct StoreKeyRequest(pub AsymmetricKeyPair);

/// Easily convert to &[AsymmetricKeyPair] from &[StoreKeyResponse].
impl Deref for StoreKeyRequest {
    type Target = AsymmetricKeyPair;
    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoreKeyResponse {
    pub key_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetKeyResponse(pub AsymmetricKeyPair);

/// Easily convert to &[AsymmetricKeyPair] from &[GetKeyResponse].
impl Deref for GetKeyResponse {
    type Target = AsymmetricKeyPair;
    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}
