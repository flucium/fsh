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

pub fn update_profile(path: &str, content: &str) -> Result<()> {
    let mut profile = read_profile(path)?;
    profile.push_str(content);
    write_profile(path, &profile)
}

pub fn exists(path: &str) -> bool {
    File::open(path).is_ok()
}
