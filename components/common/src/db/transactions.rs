use super::{
    models::{DbAsymmetricKeyPair, DbUser},
    schema::{keys::dsl as keys_dsl, users::dsl as users_dsl},
};
use crate::{
    keys::{AsymmetricKeyPair, AsymmetricKeyPairView},
    log::{info, Logger},
    result::{self, Res},
};
use diesel::{
    self, result::Error::NotFound, BelongingToDsl, ExpressionMethods, PgConnection, QueryDsl,
    RunQueryDsl, SelectableHelper,
};
use snafu::prelude::*;
use uuid::Uuid;

// Insert a new user into the database.
pub fn insert_new_user(conn: &mut PgConnection, user_id: &str) -> Res<()> {
    use super::schema::users::dsl::*;

    // Convert into internal DB type.
    let user = DbUser {
        id: user_id.to_owned(),
    };

    diesel::insert_into(users)
        .values(user)
        .execute(conn)
        .context(result::DBErrorCtx {
            message: "Transaction 'insert user' failed.",
        })?;

    return Ok(());
}

// Insert a new key pair into the database.
pub fn insert_asymmetric_key_pair(
    log: &Logger,
    conn: &mut PgConnection,
    user_id: &str,
    key_id: &Uuid,
    key_pair: &AsymmetricKeyPairView,
) -> Res<()> {
    match users_dsl::users.find(user_id).execute(conn) {
        Ok(0) => insert_new_user(conn, user_id)?,
        Ok(_) => {
            info!(log, "User already exists: '{}'", user_id);
        }
        Err(e) => {
            return Err(e).context(result::DBErrorCtx {
                message: "Operation 'insert user' failed.",
            });
        }
    }

    match keys_dsl::keys.find(key_id).execute(conn) {
        Ok(0) => (),
        Ok(_) => {
            return Err(result::Error::GenericError {
                message: format!("Key '{}' already exists.", key_id),
                source: None,
            });
        }
        Err(e) => {
            return Err(e).context(result::DBErrorCtx {
                message: "Operation 'find key' failed.",
            });
        }
    }

    // If not errored out so far -> insert the key.
    let key = DbAsymmetricKeyPair {
        id: *key_id,
        user_id: user_id.to_owned(),
        public_key: key_pair.public_key.to_owned(),
        private_key_encrypted: key_pair.private_key_encrypted.to_owned(),
    };

    {
        diesel::insert_into(keys_dsl::keys)
            .values(key)
            .execute(conn)
            .context(result::DBErrorCtx {
                message: "Transaction 'insert key' failed.",
            })?;
    }

    return Ok(());
}

pub fn get_asymmetric_key_pair(
    log: &Logger,
    conn: &mut PgConnection,
    user_id: &str,
    key_id: &Uuid,
) -> Res<AsymmetricKeyPair> {
    let res = DbAsymmetricKeyPair::belonging_to(&DbUser {
        id: user_id.to_owned(),
    })
    .filter(keys_dsl::id.eq(key_id))
    .select(DbAsymmetricKeyPair::as_select())
    .first(conn);

    let key = match res {
        Ok(key) => key,
        Err(NotFound) => {
            return Err(result::Error::GenericError {
                message: format!("Key '{}' does not exist.", key_id),
                source: None,
            });
        }
        Err(e) => {
            return Err(e).context(result::DBErrorCtx {
                message: "Operation 'find key' failed.",
            });
        }
    };

    return Ok(AsymmetricKeyPair {
        public_key: key.public_key,
        private_key_encrypted: key.private_key_encrypted,
    });
}
