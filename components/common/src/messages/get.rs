use std::ops::Deref;

use crate::keys::AsymmetricKeyPair;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetKeyResponse(pub AsymmetricKeyPair);

/// Easily convert to &[AsymmetricKeyPair] from &[GetKeyResponse].
impl Deref for GetKeyResponse {
    type Target = AsymmetricKeyPair;
    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}
