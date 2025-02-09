use crate::{error::*, result::Result};
use std::{
    env,
    ffi::OsStr,
    path::{Path, PathBuf},
    process,
};

/// Aborts the process immediately.
///
/// This function forcefully terminates the current process using `process::abort()`,
/// without running any cleanup routines.
pub fn abort() {
    process::abort()
}

/// Exits the process with a specified exit code.
///
/// This function terminates the process and returns the provided exit code
/// to the operating system.
///
/// # Arguments
/// - `code`: The exit code to return upon process termination.
pub fn exit(code: i32) {
    process::exit(code)
}

/// Changes the current working directory and returns the new path.
///
/// This function resolves the target path relative to the provided `current_path`,
/// updates the system's current working directory, and returns the resolved absolute path.
///
/// # Arguments
/// - `p`: The target directory path.
/// - `current_path`: The current directory path used as the base for resolving `p`.
///
/// # Returns
/// - `Ok(PathBuf)`: The resolved absolute path if the directory change is successful.
/// - `Err(Error::NOT_IMPLEMENTED)`: If the operation fails due to an invalid path or system error.
pub fn cd<S: AsRef<OsStr> + ?Sized>(p: &S, current_path: &Path) -> Result<PathBuf> {
    let current_path = current_path
        .canonicalize()
        .map_err(|_| Error::new(ErrorKind::InvalidPath, ""))?;

    let path = analyze(&current_path, p)?;

    env::set_current_dir(&path).map_err(|_| Error::new(ErrorKind::PermissionDenied, ""))?;

    Ok(path)
}

/// Analyzes the target directory and resolves its absolute path.
///
/// This function checks if the current directory is valid, constructs
/// an absolute path by joining it with the target path, and verifies
/// that the resulting path is a valid directory.
///
/// # Arguments
/// - `current`: The current directory.
/// - `target`: The target directory to resolve.
///
/// # Returns
/// - `Ok(PathBuf)`: The resolved absolute path.
/// - `Err(Error::NOT_IMPLEMENTED)`: If the current directory is invalid or
///   the target path is not a directory.
fn analyze<A: AsRef<OsStr> + ?Sized, B: AsRef<OsStr> + ?Sized>(
    current: &A,
    target: &B,
) -> Result<PathBuf> {
    let current = Path::new(current);

    let target = Path::new(target);

    if !current.is_dir() {
        Err(Error::new(ErrorKind::NotADirectory, ""))?
    }

    let path = Path::new(current)
        .join(Path::new(target))
        .canonicalize()
        .map_err(|_| Error::new(ErrorKind::InvalidPath, ""))?;

    if !current.is_dir() {
        Err(Error::new(ErrorKind::NotADirectory, ""))?
    }

    Ok(path)
}
