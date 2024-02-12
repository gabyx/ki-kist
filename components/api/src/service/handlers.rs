#![allow(unused_imports)] // Rocket generates pub functions which cause these warnings.

use std::ops::DerefMut;

use common::{
    db,
    keys::AsymmetricKeyPair,
    log::{debug, info},
    messages::StoreKeyRequest,
    response::{self, json, Status},
    result::ResultExt,
    rocket::WrappedUuid,
};

use rocket::{
    form::Form, routes, serde::json::Json, Build, Rocket, Shutdown, State,
};
use snafu::prelude::*;
use uuid::Uuid;

use crate::state::AppState;
use common::messages::{GetKeyResponse, StoreKeyResponse};

/// The request handler to store a key.
/// TODO: Should have a request guard `key: ApiKey` which guards against
/// non-authentication.
#[rocket::put("/api/v1/<user_id>/keys", data = "<key_pair>")]
async fn store_key(
    s: &State<AppState>,
    user_id: &str,
    key_pair: Json<StoreKeyRequest>,
) -> json::JsonResponse<StoreKeyResponse> {
    let key_id = Uuid::new_v4();

    debug!(
        s.log,
        "Storing key for user '{}' and key id '{}'", user_id, key_id
    );

    // TODO: Validate the key pair and error out if not the specific format.

    {
        debug!(s.log, "Insert into database.");
        let mut d = s.db.lock().await;

        // TODO: This call blocks the executor, put
        // it into a task or use diesel async libraries.
        db::transactions::insert_asymmetric_key_pair(
            &s.log,
            d.deref_mut(),
            user_id,
            &key_id,
            &(&key_pair as &StoreKeyRequest).0,
        )
        .log(&s.log)
        .map_err(|e| response::Error::from(e))?
    }

    return json::success!(StoreKeyResponse { key_id });
}

/// The request handler to retrieve a key.
/// TODO: Should have a request guard `key: ApiKey` which guards against
/// non-authentication.
#[rocket::get("/api/v1/<user_id>/keys/<key_id>")]
async fn get_key(
    s: &State<AppState>,
    user_id: &str,
    key_id: WrappedUuid,
) -> json::JsonResponse<GetKeyResponse> {
    debug!(
        s.log,
        "Getting key pair for user '{}' and key id '{}'",
        user_id,
        key_id.unwrap()
    );

    let key = {
        let mut d = s.db.lock().await;

        // TODO: This call blocks the executor, put
        // it into a task or use diesel async libraries.
        db::transactions::get_asymmetric_key_pair(
            &s.log,
            d.deref_mut(),
            user_id,
            key_id.unwrap(),
        )
        .log(&s.log)
        .map_err(|e| response::Error::from(e))?
    };

    return json::success!(GetKeyResponse(key));
}

#[rocket::get("/api/shutdown")]
fn shutdown(shutdown: Shutdown) {
    shutdown.notify();
}

/// Install all handlers for this application.
pub fn install_handlers(r: Rocket<Build>) -> Rocket<Build> {
    let r = r.mount("/", routes![get_key, store_key]);
    return install_debug_handlers(r);
}

#[cfg(not(feature = "debug-handlers"))]
fn install_debug_handlers(r: Rocket<Build>) -> Rocket<Build> {
    return r;
}

#[cfg(feature = "debug-handlers")]
fn install_debug_handlers(r: Rocket<Build>) -> Rocket<Build> {
    return r.mount("/", routes![shutdown]);
}
