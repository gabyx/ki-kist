use common::log::Logger;
use diesel::pg::PgConnection;
use rocket::tokio::sync::Mutex;
use std::sync::Arc;

pub struct AppState {
    // TODO: Abstract away the log. Make a simple interface in `common`.
    pub log: Arc<Logger>,

    // TODO: Abstract away db connection, if possible: Make an interface in `common`
    // such that only converter/api use the same interface and dont need to know if its postgres or
    // something else.
    pub db: Mutex<PgConnection>,
}

impl AppState {
    pub fn new(log: Arc<Logger>, db_conn: PgConnection) -> AppState {
        return AppState {
            log: log.clone(),
            db: Mutex::new(db_conn),
        };
    }
}
