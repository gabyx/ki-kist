use common::keys::{AsymmetricKeyPair, AsymmetricKeyPairView};
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StoreKeyResponse {
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetKeyResponse(pub AsymmetricKeyPair);
