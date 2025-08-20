use std::{
    ffi::OsStr,
    fs::File,
    io::{self, Read, Write},
};

use crate::{error::*, result::*};

/// The default file path for the FSH profile.
#[cfg(debug_assertions)]
pub const DEFAULT_PROFILE_PATH: &str = "./temp/profile.fsh";

/// The default file path for the FSH profile.
#[cfg(not(debug_assertions))]
pub const DEFAULT_PROFILE_PATH: &str = "~/.profile.fsh";

/// Default profile content that sets `FSH_PROMPT` to `"# "`.
pub const DEFAULT_PROFILE_CONTENT: &str = "$FSH_PROMPT = \"# \"";

/// Reads the profile file and returns its contents.
///
/// # Arguments
/// - `path` - Path to the profile file.
///
/// # Returns
/// - `Ok(String)` containing the file contents.  
/// - `Err(Error)` if the file cannot be opened or read.
pub fn read_profile<P: AsRef<OsStr>>(path: &P) -> Result<String> {
    File::open(path.as_ref())
        .map_err(|e| match e.kind() {
            io::ErrorKind::NotFound => Error::new(ErrorKind::NotFound, "profile file not found"),
            io::ErrorKind::PermissionDenied => Error::new(
                ErrorKind::PermissionDenied,
                "permission denied while accessing profile file",
            ),
            _ => Error::new(
                ErrorKind::Internal,
                "permission denied while accessing profile file",
            ),
        })
        .and_then(|mut file| {
            let mut content = String::new();
            file.read_to_string(&mut content)
                .map_err(|_| Error::new(ErrorKind::Interrupted, "failed to read profile file"))?;
            Ok(content)
        })
}

/// Writes the given content to the profile file, overwriting if it exists.
///
/// # Arguments
/// - `path` - Path to the profile file.  
/// - `content` - Data to write.
///
/// # Returns
/// - `Ok(())` if the write succeeds.  
/// - `Err(Error)` if the file cannot be created or written.
pub fn write_profile<P: AsRef<OsStr>>(path: &P, content: impl Into<String>) -> Result<()> {
    File::create(path.as_ref())
        .map_err(|_| {
            Error::new(
                ErrorKind::PermissionDenied,
                "permission denied while creating profile file",
            )
        })
        .and_then(|mut file| {
            file.write_all(content.into().as_bytes()).map_err(|_| {
                Error::new(ErrorKind::Interrupted, "failed to write to profile file")
            })?;
            Ok(())
        })
}

/// Appends the given content to the profile file.
///
/// # Arguments
/// - `path` - Path to the profile file.  
/// - `content` - Data to append.
///
/// # Returns
/// - `Ok(())` if the update succeeds.  
/// - `Err(Error)` if the file cannot be read or written.
pub fn update_profile<P: AsRef<OsStr> + ?Sized>(
    path: &P,
    content: impl Into<String>,
) -> Result<()> {
    let mut profile = read_profile(&path)?;

    profile.push_str(&content.into());

    write_profile(&path, &profile)
}

/// Checks whether the profile file exists at the given path.
///
/// # Arguments
/// - `path` - Path to check.
///
/// # Returns
/// - `true` if the file exists.  
/// - `false` otherwise.
pub fn exists<P: AsRef<OsStr> + ?Sized>(path: &P) -> bool {
    File::open(path.as_ref()).is_ok()
}
