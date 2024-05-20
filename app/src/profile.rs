use fsh_common::{Error, ErrorKind, Result};
use std::{
    fs::File,
    io::{self, Read, Write},
    os::unix::fs::MetadataExt,
    path::Path,
};

/// Default profile path.
#[cfg(not(debug_assertions))]
pub const DEFAULT_PROFILE_PATH: &str = "~/.profile.fsh";

/// Default profile content.
#[cfg(debug_assertions)]
pub const DEFAULT_PROFILE_PATH: &str = "./temp/profile.fsh";

/// Default profile content.
#[cfg(not(debug_assertions))]
pub const DEFAULT_PROFILE_CONTENT: &str = "FSH_PROMPT=\"# \"\nFSH_HISTORY = true\nFSH_HISTORY_SIZE = 1000\nFSH_HISTORY_FILE = \"~/.fsh_history\"";

/// Default profile content.
#[cfg(debug_assertions)]
pub const DEFAULT_PROFILE_CONTENT: &str = "FSH_PROMPT=\"# \"\nFSH_HISTORY = true\nFSH_HISTORY_SIZE = 1000\nFSH_HISTORY_FILE = \"../temp/fsh_history\"";

/// Create a new profile.
///
/// or open it if it already exists.
///
/// Permissions: Read, Write, Create.
///
/// # Errors
/// - `ErrorKind::PermissionDenied`: Permission denied.
/// - `ErrorKind::Other`: Other error.
pub fn create<P: AsRef<Path>>(path: P) -> Result<File> {
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
}

/// Open a profile.
///
/// Permissions: Read, Write.
///
/// # Errors
/// - `ErrorKind::NotFound`: Profile not found.
/// - `ErrorKind::PermissionDenied`: Permission denied.
/// - `ErrorKind::Other`: Other error.
pub fn open<P: AsRef<Path>>(path: P) -> Result<File> {
    File::options()
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
        })
}

/// Write to a profile.
///
/// # Errors
/// - `ErrorKind::PermissionDenied`: Permission denied.
/// - `ErrorKind::Interrupted`: Interrupted.
/// - `ErrorKind::Other`: Other error.
pub fn write(file: &mut File, contents: &str) -> Result<()> {
    file.write_all(contents.as_bytes())
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

    Ok(())
}

/// Read from a profile.
///
/// # Errors
/// - `ErrorKind::PermissionDenied`: Permission denied.
/// - `ErrorKind::Interrupted`: Interrupted.
/// - `ErrorKind::Other`: Other error.
pub fn read(file: &mut File) -> Result<String> {
    let mut contents = String::new();
    file.read_to_string(&mut contents)
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

    Ok(contents)
}

/// Check if a profile exists.
pub fn exists<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().exists()
}

/// Get the mode of a profile.
fn mode<P: AsRef<Path>>(path: P) -> Result<u32> {
    if !exists(&path) {
        Err(Error::new(ErrorKind::NotFound, "profile not found"))?;
    }

    let metadata = path
        .as_ref()
        .metadata()
        .map_err(|_| Error::new(ErrorKind::Other, "other error"))?;

    Ok(metadata.mode())
}

/// Check if a profile is readable.
pub fn is_readable<P: AsRef<Path>>(path: P) -> bool {
    let mode = mode(path).unwrap_or(0);
    mode & 0o400 != 0
}

/// Check if a profile is writable.
pub fn is_writable<P: AsRef<Path>>(path: P) -> bool {
    let mode = mode(path).unwrap_or(0);
    mode & 0o200 != 0
}

/// Get profile path parent directory
pub fn parent<P: AsRef<Path>>(path: P) -> Result<String> {
    path.as_ref()
        .parent()
        .map(|p| p.to_str().unwrap().to_string())
        .ok_or(Error::new(ErrorKind::Other, "other error"))
}

#[cfg(test)]
mod tests {
    /*
        This test assumes that there is a directory named 'temp' in the parent directory of './app'.
        in other words, for './app', it would be '../temp'.

        Command: e.g. cargo test --package fsh --bin fsh -- profile::tests::test_create --exact --show-output
                      cargo test --package fsh --bin fsh -- profile::tests --show-output
    */

    use super::*;
    use std::fs;

    #[test]
    fn test_create() {
        let path = "../temp/test_create";

        // Remove the file if it exists.
        let _ = fs::remove_file(path);

        // Create a new profile.
        let profile = create(path);

        // Check if the profile was created successfully.
        assert!(profile.is_ok());
        assert!(exists(path));

        // Remove the file.
        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_open() {
        let path = "../temp/test_open";

        // Remove the file if it exists.
        let _ = fs::remove_file(path);

        // Open a profile that does not exist.
        let profile = open(path);

        // Check if the profile was opened successfully.
        // is_err() returns true if the result is an error.
        assert!(profile.is_err());
        assert!(!exists(path));

        // Create a new profile.
        // Write some content to the file.
        let _ = fs::write(path, "test");

        // Open the profile.
        let profile = open(path);

        // Check if the profile was opened successfully.
        assert!(profile.is_ok());
        assert!(exists(path));

        // Remove the file.
        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_write() {
        let path = "../temp/test_write";

        // Remove the file if it exists.
        let _ = fs::remove_file(path);

        // Create a new profile.
        let mut file = create(path).unwrap();

        // Write to the profile.
        let result = write(&mut file, "test");

        // Check if the write operation was successful.
        // Read the contents of the file and check if it matches the written content.
        assert!(result.is_ok());
        assert_eq!(fs::read_to_string(path).unwrap(), "test");

        // Remove the file.
        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_read() {
        let path = "../temp/test_read";

        // Remove the file if it exists.
        let _ = fs::remove_file(path);

        // Create a new profile.
        // Write some content to the file.
        let _ = fs::write(path, "test");

        // Open the profile.
        let mut file = open(path).unwrap();

        // Read the contents of the profile.
        let result = read(&mut file);

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

    #[test]
    fn test_mode() {
        let path = "../temp/test_mode";

        // Remove the file if it exists.
        let _ = fs::remove_file(path);

        // Check if the file exists.
        assert!(!exists(path));

        // Create a new file.
        let _ = fs::write(path, "test");

        // Check if the file exists.
        assert!(exists(path));

        // Check the mode of the file.
        let mode = mode(path);

        // Check if the mode was retrieved successfully.
        assert!(mode.is_ok());

        // Remove the file.
        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_is_readable() {
        let path = "../temp/test_is_readable";

        // Remove the file if it exists.
        let _ = fs::remove_file(path);

        // Check if the file is readable.
        assert!(!is_readable(path));

        // Create a new file.
        let _ = fs::write(path, "test");

        // Check if the file is readable.
        assert!(is_readable(path));

        // Remove the file.
        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_is_writable() {
        let path = "../temp/test_is_writable";

        // Remove the file if it exists.
        let _ = fs::remove_file(path);

        // Check if the file is writable.
        assert!(!is_writable(path));

        // Create a new file.
        let _ = fs::write(path, "test");

        // Check if the file is writable.
        assert!(is_writable(path));

        // Remove the file.
        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_parent() {
        let path = "../temp/test_parent";

        // Remove the file if it exists.
        let _ = fs::remove_file(path);

        // Check if the parent directory exists.
        assert!(parent(path).is_ok());

        // Create a new file.
        let _ = fs::write(path, "test");

        // Check if the parent directory exists.
        assert!(parent(path).is_ok());

        // Check parent directory path.
        assert_eq!(parent(path).unwrap(), "../temp");

        // Remove the file.
        let _ = fs::remove_file(path);
    }
}
