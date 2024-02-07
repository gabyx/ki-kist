use common::{
    log::{self},
    response,
};
use rocket::fs::TempFile;

pub async fn store_key(
    log: &log::Logger,
    file: &mut TempFile<'_>,
    name: &str,
) -> Result<String, response::Error> {
    // let content_type = match file.content_type() {
    //     Some(c) => c.to_string(),
    //     None => {
    //         return Err(response::error!(
    //             Status::BadRequest,
    //             "No content type given.",
    //         ))
    //     }
    // };
    //
    // match content_type.as_str() {
    //     "text/markdown" | "application/gzip" => (),
    //     _ => {
    //         return Err(response::error!(
    //             Status::BadRequest,
    //             "Content type '{}' files are not supported (only 'text/markdown' and 'application/gzip').",
    //             content_type
    //         ));
    //     }
    // };
    //
    // let path = storage.pre_store();
    // info!(log, "Persist upload to temporary file '{:?}'.", path.path());
    //
    // file.copy_to(path.path())
    //     .await
    //     .log(log)
    //     .context(result::IOErrorCtx)?;
    //
    // let digest = storage
    //     .store(&log, path.finalize())
    //     .await
    //     .log(log)
    //     .context(result::IOErrorCtx)?;
    //
    // return Ok(JobBundle::new(&name, &digest, &content_type));
    Ok("string".to_owned())
}
