use fsh_common::{Error, ErrorKind, Result};
use std::{
    fs,
    io::{self, Read, Write},
    path,
};

/// Default profile path.
#[cfg(not(debug_assertions))]
pub const DEFAULT_PROFILE_PATH: &str = "~/.profile.fsh";

/// Default profile content.
#[cfg(debug_assertions)]
pub const DEFAULT_PROFILE_PATH: &str = "./temp/profile.fsh";

/// Default profile content.
#[cfg(not(debug_assertions))]
const DEFAULT_PROFILE_CONTENT: &str = "FSH_PROMPT=\"# \"\nFSH_HISTORY = true\nFSH_HISTORY_SIZE = 1000\nFSH_HISTORY_FILE = \"~/.fsh_history\"";

/// Default profile content.
#[cfg(debug_assertions)]
const DEFAULT_PROFILE_CONTENT: &str = "FSH_PROMPT=\"# \"\nFSH_HISTORY = true\nFSH_HISTORY_SIZE = 1000\nFSH_HISTORY_FILE = \"../temp/fsh_history\"";

/// Creates a new profile at the specified path.
///
/// Open the profile if it already exists
///
/// # Arguments
/// - `path` - The path to the profile.
///
/// # Returns
/// The default profile content.
///
/// # Errors
/// - `ErrorKind::PermissionDenied`: The profile (file) could not be created due to a permission error.
/// - `ErrorKind::Interrupted`: The profile (file) could not be created due to an interrupted operation.
/// - `ErrorKind::Other`: The profile (file) could not be created due to an unknown error.
pub fn create<P: AsRef<path::Path>>(path: P) -> Result<String> {
    fs::File::options()
        .create(true)
        .write(true)
        .read(true)
        .open(path)
        .map_err(|err| match err.kind() {
            // Permission denied
            io::ErrorKind::PermissionDenied => {
                Error::new(ErrorKind::PermissionDenied, "failed to create profile")
            }

            // Other error
            _ => Error::new(ErrorKind::Other, "unknown error"),
        })?
        .write_all(DEFAULT_PROFILE_CONTENT.as_bytes())
        .map_err(|err| {
            match err.kind() {
                // Interrupteds
                io::ErrorKind::Interrupted => Error::new(
                    ErrorKind::Interrupted,
                    "writing default content was interrupted",
                ),

                // Other error
                _ => Error::new(ErrorKind::Other, "unknown error"),
            }
        })?;

    Ok(DEFAULT_PROFILE_CONTENT.to_string())
}

/// Reads the content of the profile at the specified path.
///
/// # Arguments
/// - `path` - The path to the profile.
///
/// # Returns
/// The read content of the profile.
///
/// # Errors
/// - `ErrorKind::NotFound`: The profile (file) could not be read because it was not found.
/// - `ErrorKind::PermissionDenied`: The profile (file) could not be read due to a permission error.
/// - `ErrorKind::Interrupted`: The profile (file) could not be read due to an interrupted operation.
/// - `ErrorKind::Other`: The profile (file) could not be read due to an unknown error.
pub fn read<P: AsRef<path::Path>>(path: P) -> Result<String> {
    if path.as_ref().exists() == false {
        Err(Error::new(ErrorKind::NotFound, "failed to open profile"))?;
    };

    let mut buffer = String::new();

    fs::File::options()
        .read(true)
        .open(path)
        .map_err(|err| match err.kind() {
            // Permission denied
            io::ErrorKind::PermissionDenied => {
                Error::new(ErrorKind::PermissionDenied, "failed to open profile")
            }

            // Other error
            _ => Error::new(ErrorKind::Other, "unknown error"),
        })?
        .read_to_string(&mut buffer)
        .map_err(|err| {
            match err.kind() {
                // Interrupteds
                io::ErrorKind::Interrupted => {
                    Error::new(ErrorKind::Interrupted, "reading profile was interrupted")
                }

                // Other error
                _ => Error::new(ErrorKind::Other, "unknown error"),
            }
        })?;

    Ok(buffer)
}

/*
/// Writes the specified content to the profile at the specified path.
///  
/// If the profile does not exist, it will be created.
///
/// # Arguments
/// - `path` - The path to the profile.
///
/// # Returns
/// if write is successful, returns `Ok(())`.
///
/// # Errors
/// - `ErrorKind::PermissionDenied`: The profile (file) could not be written to due to a permission error.
/// - `ErrorKind::Interrupted`: The profile (file) could not be written to due to an interrupted operation.
/// - `ErrorKind::Other`: The profile (file) could not be written to due to an unknown error.
*/
// pub fn write<P: AsRef<path::Path>>(path: P, content: &str) -> Result<()> {
//     fs::File::options()
//         .create(true)
//         .write(true)
//         .open(path)
//         .map_err(|err| match err.kind() {
//             // Permission denied
//             io::ErrorKind::PermissionDenied => {
//                 Error::new(ErrorKind::PermissionDenied, "failed to create profile")
//             }

//             // Other error
//             _ => Error::new(ErrorKind::Other, "unknown error"),
//         })?
//         .write_all(content.as_bytes())
//         .map_err(|err| {
//             match err.kind() {
//                 // Interrupteds
//                 io::ErrorKind::Interrupted => Error::new(
//                     ErrorKind::Interrupted,
//                     "writing default content was interrupted",
//                 ),

//                 // Other error
//                 _ => Error::new(ErrorKind::Other, "unknown error"),
//             }
//         })
// }

/// Returns true if the profile exists.
///
/// # Arguments
/// - `path` - The path to the profile.
///
/// # Returns
/// `true` if the profile exists; otherwise, `false`.
pub fn exists<P: AsRef<path::Path>>(path: P) -> bool {
    path.as_ref().exists()
}

/*
/// Removes the profile at the specified path.
///
/// # Arguments
/// - `path` - The path to the profile.
///
/// # Returns
/// if removal is successful, returns `Ok(())`.
///
/// # Errors
/// - `ErrorKind::NotFound`: The profile (file) could not be removed because it was not found.
/// - `ErrorKind::PermissionDenied`: The profile (file) could not be removed due to a permission error.
/// - `ErrorKind::Other`: The profile (file) could not be removed due to an unknown error.
*/
// pub fn remove<P: AsRef<path::Path>>(path: P) -> Result<()> {
//     let path = path.as_ref();

//     if path.exists() == false {
//         Err(Error::new(ErrorKind::NotFound, "failed to remove profile"))?;
//     };

//     std::fs::remove_file(path).map_err(|err| match err.kind() {
//         // Permission denied
//         io::ErrorKind::PermissionDenied => {
//             Error::new(ErrorKind::PermissionDenied, "failed to remove profile")
//         }

//         // Other error
//         _ => Error::new(ErrorKind::Other, "unknown error"),
//     })
// }

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
        // Test path
        const PATH: &str = "../temp/test_create.fsh";

        // Act and assert
        assert_eq!(create(PATH).is_ok(), true);

        // Clean up
        fs::remove_file(PATH).unwrap();
    }

    #[test]
    fn test_read() {
        // Test path
        const PATH: &str = "../temp/test_read.fsh";

        // Create file with test contents
        let _ = fs::File::options()
            .create(true)
            .write(true)
            .open(PATH)
            .unwrap()
            .write_all(b"Hello, World!")
            .unwrap();

        // Act and assert
        assert_eq!(read(PATH).is_ok(), true);
        assert_eq!(read(PATH).unwrap(), "Hello, World!");

        // Clean up
        let _ = fs::remove_file(PATH);
    }

    // #[test]
    // fn test_write() {
    //     // Test path
    //     const PATH: &str = "../temp/test_write.fsh";

    //     // Act and assert.
    //     assert_eq!(write(PATH, "Hello, World!").is_ok(), true);

    //     // Was write successful?
    //     assert_eq!(read(PATH).unwrap(), "Hello, World!");

    //     // Clean up
    //     let _ = fs::remove_file(PATH);
    // }

    #[test]
    fn test_exists() {
        // Test path
        const PATH: &str = "../temp/test_exists.fsh";

        // Create file
        let _ = fs::File::options()
            .create(true)
            .write(true)
            .open(PATH)
            .unwrap()
            .write_all(b"Hello, World!")
            .unwrap();

        // Act and assert
        assert_eq!(exists(PATH), true);

        // Clean up
        let _ = fs::remove_file(PATH);
    }

    // #[test]
    // fn test_remove() {
    //     // Test path
    //     const PATH: &str = "../temp/test_remove.fsh";

    //     // Create file
    //     let _ = fs::File::options()
    //         .create(true)
    //         .write(true)
    //         .open(PATH)
    //         .unwrap()
    //         .write_all(b"Hello, World!")
    //         .unwrap();

    //     // Act and assert
    //     assert_eq!(remove(PATH).is_ok(), true);

    //     // Clean up
    //     let _ = fs::remove_file(PATH);
    // }
}
