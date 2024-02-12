use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AsymmetricKeyPair {
    pub public_key: String,
    pub private_key_encrypted: String,
}
