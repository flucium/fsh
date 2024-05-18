use fsh_common::{Error, ErrorKind, Result};
use std::{
    fs::File,
    io::{self, Read, Write},
    path::Path,
};

/// The default path to the profile.
#[cfg(not(debug_assertions))]
pub const DEFAULT_PROFILE_PATH: &str = "~/.profile.fsh";

/// The default path to the profile.
#[cfg(debug_assertions)]
pub const DEFAULT_PROFILE_PATH: &str = "./temp/profile.fsh";

#[cfg(not(debug_assertions))]
pub const DEFAULT_PROFILE_CONTENT: &str = "FSH_PROMPT=\"# \"\nFSH_HISTORY = true\nFSH_HISTORY_SIZE = 1000\nFSH_HISTORY_FILE = \"~/.fsh_history\"";

#[cfg(debug_assertions)]
pub const DEFAULT_PROFILE_CONTENT: &str = "FSH_PROMPT=\"# \"\nFSH_HISTORY = true\nFSH_HISTORY_SIZE = 1000\nFSH_HISTORY_FILE = \"../temp/fsh_history\"";


/// An object representing a profile.
pub struct Profile(File);

impl Profile {
    /// Create a new profile instance at the given path.
    ///
    /// Opens the file at the given path with read and write permissions. create a new file, or open it if it already exists.
    ///
    /// # Errors
    /// Returns an error if the file cannot be created or opened.
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        File::options()
            .create(true)
            .write(true)
            .read(true)
            .open(path)
            .map_err(|err| match err.kind() {
                // Permission denied
                io::ErrorKind::PermissionDenied => {
                    Error::new(ErrorKind::PermissionDenied, "permission denied")
                }

                // Other error
                _ => Error::new(ErrorKind::Other, "other error"),
            })
            .map(Self)
    }

    /// Create a new profile instance at the given path.
    ///
    /// Opens the file at the given path with read and write permissions.
    ///
    /// # Errors
    /// Returns an error if the file does not exist or cannot be opened.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        // Open the file at the given path
        let file = File::options()
            .write(true)
            .read(true)
            .open(path)
            .map_err(|err| match err.kind() {
                // Not found
                io::ErrorKind::NotFound => Error::new(ErrorKind::NotFound, "Profile not found"),

                // Permission denied
                io::ErrorKind::PermissionDenied => {
                    Error::new(ErrorKind::PermissionDenied, "permission denied")
                }

                // Other error
                _ => Error::new(ErrorKind::Other, "other error"),
            })?;

        // Return the Profile instance
        Ok(Self(file))
    }

    /// Write the given contents to the profile.
    ///
    /// # Errors
    /// Returns an error if the contents cannot be written to the file.
    /// - `PermissionDenied` - The file cannot be written to due to insufficient permissions
    /// - `Interrupted` - The write operation was interrupted
    /// - `Other` - Any other error occurred
    pub fn write(&mut self, contents: &str) -> Result<()> {
        // Write the contents to the file
        self.0
            .write_all(contents.as_bytes())
            .map_err(|err| match err.kind() {
                // Permission denied
                io::ErrorKind::PermissionDenied => {
                    Error::new(ErrorKind::PermissionDenied, "permission denied")
                }

                // Interrupted
                io::ErrorKind::Interrupted => Error::new(ErrorKind::Interrupted, "interrupted"),

                // Other error
                _ => Error::new(ErrorKind::Other, "other error"),
            })?;

        // Return success
        Ok(())
    }

    /// Read the contents of the profile.
    ///
    /// # Errors
    /// Returns an error if the contents cannot be read from the file.
    /// - `PermissionDenied` - The file cannot be read from due to insufficient permissions
    /// - `Interrupted` - The read operation was interrupted
    /// - `Other` - Any other error occurred
    pub fn read(&mut self) -> Result<String> {
        // Read the contents of the file
        let mut contents = String::new();
        self.0
            .read_to_string(&mut contents)
            .map_err(|err| match err.kind() {
                // Permission denied
                io::ErrorKind::PermissionDenied => {
                    Error::new(ErrorKind::PermissionDenied, "permission denied")
                }

                // Interrupted
                io::ErrorKind::Interrupted => Error::new(ErrorKind::Interrupted, "interrupted"),

                // Other error
                _ => Error::new(ErrorKind::Other, "other error"),
            })?;

        // Return the contents
        Ok(contents)
    }
}

/// Returns true if the path exists and false otherwise.
pub fn exists<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().exists()
}

#[cfg(test)]
mod tests {
    /*
        This test assumes that there is a directory named 'temp' in the parent directory of './app'.
        in other words, for './app', it would be '../temp'.

        Command: e.g. cargo test --package fsh --bin fsh -- profile::tests::test_profile_new --exact --show-output
                      cargo test --package fsh --bin fsh -- profile::tests --show-output
    */

    use super::*;
    use std::fs;

    #[test]
    fn test_profile_new() {
        let path = "../temp/test_profile_new";

        // Remove the file if it exists.
        let _ = fs::remove_file(path);

        // Create a new profile.
        let profile = Profile::new(path);

        // Check if the profile was created successfully.
        assert!(profile.is_ok());
        assert!(exists(path));

        // Remove the file.
        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_profile_open() {
        let path = "../temp/test_profile_open";

        // Remove the file if it exists.
        let _ = fs::remove_file(path);

        // Open a profile that does not exist.
        let profile = Profile::open(path);

        // Check if the profile was opened successfully.
        // is_err() returns true if the result is an error.
        assert!(profile.is_err());
        assert!(!exists(path));

        // Create a new profile.
        // Write some content to the file.
        let _ = fs::write(path, "test");

        // Open the profile.
        let profile = Profile::open(path);

        // Check if the profile was opened successfully.
        assert!(profile.is_ok());
        assert!(exists(path));

        // Remove the file.
        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_profile_write() {
        let path = "../temp/test_profile_write";

        // Remove the file if it exists.
        let _ = fs::remove_file(path);

        // Create a new profile.
        let mut profile = Profile::new(path).unwrap();

        // Write to the profile.
        let result = profile.write("test");

        // Check if the write operation was successful.
        // Read the contents of the file and check if it matches the written content.
        assert!(result.is_ok());
        assert_eq!(fs::read_to_string(path).unwrap(), "test");

        // Remove the file.
        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_profile_read() {
        let path = "../temp/test_profile_read";

        // Remove the file if it exists.
        let _ = fs::remove_file(path);

        // Create a new profile.
        // Write some content to the file.
        let _ = fs::write(path, "test");

        // Open the profile.
        let mut profile = Profile::open(path).unwrap();

        // Read the contents of the profile.
        let result = profile.read();

        // Check if the read operation was successful.
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test");

        // Remove the file.
        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_exists() {
        let path = "../temp/test_exists";

        // Remove the file if it exists.
        let _ = fs::remove_file(path);

        // Check if the file exists.
        assert!(!exists(path));

        // Create a new file.
        let _ = fs::write(path, "test");

        // Check if the file exists.
        assert!(exists(path));

        // Remove the file.
        let _ = fs::remove_file(path);
    }
}
