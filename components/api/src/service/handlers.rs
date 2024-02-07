#![allow(unused_imports)] // Rocket generates pub functions which cause these warnings.

use std::ops::DerefMut;

use common::{
    db,
    log::info,
    response,
    response::{json, Status},
    result::ResultExt,
    rocket::WrappedUuid,
};
use rocket::{form::Form, routes, Build, Rocket, Shutdown, State};
use snafu::prelude::*;

use crate::{persist, state::AppState};

#[rocket::put("/api/user/<user_id>/store/<key_id>")]
async fn store_key(
    s: &State<AppState>,
    user_id: WrappedUuid,
    key_id: WrappedUuid,
) -> json::JsonResponse<String> {
    json::success!("Works".to_owned())
}

#[rocket::get("/api/shutdown")]
fn shutdown(shutdown: Shutdown) {
    shutdown.notify();
}

/// Install all handlers for this application.
pub fn install_handlers(r: Rocket<Build>) -> Rocket<Build> {
    let r = r.mount("/", routes![store_key]);
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
