use std::{ffi::OsString, path::PathBuf};

// Annoyingly `p.with_extension("sig")` will turn `foo.bar` into `foo.sig`. This
// avoids that issue (and without requiring the path be UTF-8), but is kind of
// tedious. Note that `ext` should be something like `"sig"` and not `".sig"`.
// TODO: Add tests. Taken from https://github.com/badboy/signify-rs/blob/main/signify/src/main.rs#L71
pub fn add_extension(p: impl Into<PathBuf>, ext: &str) -> PathBuf {
    let mut path: PathBuf = p.into();
    let mut name: OsString = path.file_name().unwrap_or_default().to_owned();
    name.push(".");
    name.push(ext);
    path.set_file_name(name);
    path
}
