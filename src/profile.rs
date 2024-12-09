use crate::{
    error::{Error, ErrorKind},
    result::Result,
};
use std::{
    fs::File,
    io::{Read, Write},
};

#[cfg(debug_assertions)]
pub const DEFAULT_PROFILE_PATH: &str = "./temp/profile.fsh";

#[cfg(not(debug_assertions))]
pub const DEFAULT_PROFILE_PATH: &str = "~/.profile.fsh";

pub const DEFAULT_PROFILE: &str = "";

/// Reads the content of a profile file.
///
/// This function attempts to open and read the specified profile file. If the file does not exist
/// or cannot be read, an error is returned.
///
/// # Arguments
/// - `path`: The file path to the profile.
///
/// # Returns
/// - `Ok(String)`: The content of the profile file as a string.
/// - `Err(Error)`: An error if the file is not found or cannot be read.
pub fn read_profile(path: &str) -> Result<String> {
    File::open(path)
        .map_err(|_| Error::new(ErrorKind::NotFound, format!("profile not found: {}", path)))
        .and_then(|mut file| {
            let mut content = String::new();
            file.read_to_string(&mut content)
                .map_err(|_| Error::new(ErrorKind::Internal, "failed to read file".to_string()))?;
            Ok(content)
        })
}

/// Writes content to a profile file.
///
/// This function creates or overwrites the specified profile file with the provided content. If
/// the file cannot be created or written to, an error is returned.
///
/// # Arguments
/// - `path`: The file path to the profile.
/// - `content`: The content to write to the profile.
///
/// # Returns
/// - `Ok(())`: If the write operation is successful.
/// - `Err(Error)`: An error if the file cannot be created or written to.
pub fn write_profile(path: &str, content: &str) -> Result<()> {
    File::create(path)
        .map_err(|_| {
            Error::new(
                ErrorKind::PermissionDenied,
                format!("failed to create file: {}", path),
            )
        })
        .and_then(|mut file| {
            file.write_all(content.as_bytes())
                .map_err(|_| Error::new(ErrorKind::Internal, "failed to write file".to_string()))?;
            Ok(())
        })
}

/// Appends content to an existing profile file.
///
/// This function reads the existing profile file, appends the provided content, and writes the
/// updated content back to the file. If the file does not exist or cannot be written to, an error
/// is returned.
///
/// # Arguments
/// - `path`: The file path to the profile.
/// - `content`: The content to append to the profile.
///
/// # Returns
/// - `Ok(())`: If the update operation is successful.
/// - `Err(Error)`: An error if the file cannot be read or written to.
pub fn update_profile(path: &str, content: &str) -> Result<()> {
    let mut profile = read_profile(path)?;
    profile.push_str(content);
    write_profile(path, &profile)
}

/// Checks if a profile file exists.
///
/// This function checks if the specified profile file exists and is accessible.
///
/// # Arguments
/// - `path`: The file path to the profile.
///
/// # Returns
/// - `true`: If the file exists and can be opened.
/// - `false`: If the file does not exist or cannot be opened.
pub fn exists(path: &str) -> bool {
    File::open(path).is_ok()
}
