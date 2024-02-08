use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AsymmetricKeyPair {
    pub public_key: String,
    pub private_key_encrypted: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AsymmetricKeyPairView<'a> {
    pub public_key: &'a str,
    pub private_key_encrypted: &'a str,
}
