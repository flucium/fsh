use std::{
    ffi::OsStr,
    fs::File,
    io::{Read, Write},
};

use crate::{error::*, result::*};

/// The default path to the FSH profile file.
#[cfg(debug_assertions)]
pub const DEFAULT_PROFILE_PATH: &str = "./temp/profile.fsh";

/// The default path to the FSH profile file.
#[cfg(not(debug_assertions))]
pub const DEFAULT_PROFILE_PATH: &str = "~/.profile.fsh";

pub const DEFAULT_PROFILE_CONTENT: &str = "$FSH_PROMPT = \"# \"";

/// Reads the contents of the profile file at the specified path.
///
/// Opens the file in read-only mode and returns its entire contents as a `String`.
///
/// # Arguments
/// - `path`: The path to the profile file.
///
/// # Returns
/// - `Ok(String)` containing the file content if successful.
/// - `Err(Error::NOT_IMPLEMENTED)` if the file cannot be opened or read.
pub fn read_profile<P: AsRef<OsStr>>(path: &P) -> Result<String> {
    File::open(path.as_ref())
        .map_err(|_| Error::NOT_IMPLEMENTED)
        .and_then(|mut file| {
            let mut content = String::new();
            file.read_to_string(&mut content)
                .map_err(|_| Error::NOT_IMPLEMENTED)?;
            Ok(content)
        })
}

/// Writes the given content to the profile file at the specified path,
/// replacing any existing content.
///
/// Opens the file in write mode, truncating it if it already exists.
///
/// # Arguments
/// - `path`: The path to the profile file.
/// - `content`: The content to write to the file.
///
/// # Returns
/// - `Ok(())` if the write is successful.
/// - `Err(Error::NOT_IMPLEMENTED)` if the file cannot be created or written.
pub fn write_profile<P: AsRef<OsStr>>(path: &P, content: impl Into<String>) -> Result<()> {
    File::create(path.as_ref())
        .map_err(|_| Error::NOT_IMPLEMENTED)
        .and_then(|mut file| {
            file.write_all(content.into().as_bytes())
                .map_err(|_| Error::NOT_IMPLEMENTED)?;
            Ok(())
        })
}

/// Appends the given content to the end of the profile file at the specified path.
///
/// Reads the existing profile, appends the new content, and writes it back.
///
/// # Arguments
/// - `path`: The path to the profile file.
/// - `content`: The content to append.
///
/// # Returns
/// - `Ok(())` if the update is successful.
/// - `Err(Error::NOT_IMPLEMENTED)` if reading or writing the file fails.
pub fn update_profile<P: AsRef<OsStr> + ?Sized>(
    path: &P,
    content: impl Into<String>,
) -> Result<()> {
    let mut profile = read_profile(&path)?;

    profile.push_str(&content.into());

    write_profile(&path, &profile)
}

/// Returns `true` if a file exists at the given path and can be opened.
///
/// This is a simple existence check using `File::open`.
///
/// # Arguments
/// - `path`: The path to the file to check.
///
/// # Returns
/// - `true` if the file exists and is readable.
/// - `false` otherwise.
pub fn exists<P: AsRef<OsStr> + ?Sized>(path: &P) -> bool {
    File::open(path.as_ref()).is_ok()
}
