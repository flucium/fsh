use crate::{error::*, result::Result, utils};
use std::{
    env,
    ffi::OsStr,
    path::{Path, PathBuf},
    process,
};

/// Immediately aborts the current process without cleanup.
///
/// This causes the process to terminate abnormally and may generate a core dump,
/// depending on the system configuration.
///
/// This function does not return.
pub fn abort() {
    process::abort()
}

/// Exits the current process with the specified exit code.
///
/// An exit code of `0` typically indicates success, while any non-zero
/// value indicates an error or abnormal termination.
///
/// This function does not return.
///
/// # Arguments
/// - `code`: The process exit code.
pub fn exit(code: i32) {
    process::exit(code)
}

/// Changes the current working directory relative to a given base path.
///
/// This function resolves the given path `p` relative to the `current_path`,
/// sets it as the current working directory, and returns the resolved absolute path.
///
/// # Arguments
/// - `p`: The target path to change to.
/// - `current_path`: The base directory for resolving relative paths.
///
/// # Returns
/// - `Ok(PathBuf)` containing the new working directory if successful.
/// - `Err(Error::NOT_IMPLEMENTED)` if path resolution or directory change fails.
pub fn cd<S: AsRef<OsStr> + ?Sized>(p: &S, current_path: &Path) -> Result<PathBuf> {
    let path = utils::path::resolve_relative(&current_path, p)?;

    env::set_current_dir(&path).map_err(|_| Error::NOT_IMPLEMENTED)?;

    Ok(path)
}
