use crate::log::{info, Logger};
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AsymmetricKeyPair {
    pub public_key: String,
    pub private_key_encrypted: String,
}

impl AsymmetricKeyPair {
    pub fn log(&self, log: &Logger) {
        info!(
            log,
            "Public Key File 'key.pub' content:\n{}", self.public_key
        );

        info!(
            log,
            "Private Key File 'key.prv' content:\n{}",
            self.private_key_encrypted
        );
    }
}
