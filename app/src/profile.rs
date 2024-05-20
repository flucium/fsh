use fsh_common::{Error, ErrorKind, Result};
use std::{
    fs::File,
    io::{self, Read, Write},
    path::Path,
};

#[cfg(not(debug_assertions))]
pub const DEFAULT_PROFILE_PATH: &str = "~/.profile.fsh";

#[cfg(debug_assertions)]
pub const DEFAULT_PROFILE_PATH: &str = "./temp/profile.fsh";

#[cfg(not(debug_assertions))]
pub const DEFAULT_PROFILE_CONTENT: &str = "FSH_PROMPT=\"# \"\nFSH_HISTORY = true\nFSH_HISTORY_SIZE = 1000\nFSH_HISTORY_FILE = \"~/.fsh_history\"";

#[cfg(debug_assertions)]
pub const DEFAULT_PROFILE_CONTENT: &str = "FSH_PROMPT=\"# \"\nFSH_HISTORY = true\nFSH_HISTORY_SIZE = 1000\nFSH_HISTORY_FILE = \"../temp/fsh_history\"";

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

pub fn exists<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().exists()
}

// pub struct Profile(File);

// impl Profile {
//     // pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
//     //     File::options()
//     //         .create(true)
//     //         .write(true)
//     //         .read(true)
//     //         .open(path)
//     //         .map_err(|err| match err.kind() {
//     //             // Permission denied
//     //             io::ErrorKind::PermissionDenied => {
//     //                 Error::new(ErrorKind::PermissionDenied, "permission denied")
//     //             }

//     //             // Other error
//     //             _ => Error::new(ErrorKind::Other, "other error"),
//     //         })
//     //         .map(Self)
//     // }

//     pub fn create<P: AsRef<Path>>(path: P) -> Result<Self> {
//         File::options()
//             .create(true)
//             .write(true)
//             .read(true)
//             .open(path)
//             .map_err(|err| match err.kind() {
//                 // Permission denied
//                 io::ErrorKind::PermissionDenied => {
//                     Error::new(ErrorKind::PermissionDenied, "permission denied")
//                 }

//                 // Other error
//                 _ => Error::new(ErrorKind::Other, "other error"),
//             })
//             .map(Self)
//     }

//     pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
//         // Open the file at the given path
//         let file = File::options()
//             .write(true)
//             .read(true)
//             .open(path)
//             .map_err(|err| match err.kind() {
//                 // Not found
//                 io::ErrorKind::NotFound => Error::new(ErrorKind::NotFound, "Profile not found"),

//                 // Permission denied
//                 io::ErrorKind::PermissionDenied => {
//                     Error::new(ErrorKind::PermissionDenied, "permission denied")
//                 }

//                 // Other error
//                 _ => Error::new(ErrorKind::Other, "other error"),
//             })?;

//         // Return the Profile instance
//         Ok(Self(file))
//     }

//     pub fn write(&mut self, contents: &str) -> Result<()> {
//         // Write the contents to the file
//         self.0
//             .write_all(contents.as_bytes())
//             .map_err(|err| match err.kind() {
//                 // Permission denied
//                 io::ErrorKind::PermissionDenied => {
//                     Error::new(ErrorKind::PermissionDenied, "permission denied")
//                 }

//                 // Interrupted
//                 io::ErrorKind::Interrupted => Error::new(ErrorKind::Interrupted, "interrupted"),

//                 // Other error
//                 _ => Error::new(ErrorKind::Other, "other error"),
//             })?;

//         // Return success
//         Ok(())
//     }

//     pub fn read(&mut self) -> Result<String> {
//         // Read the contents of the file
//         let mut contents = String::new();
//         self.0
//             .read_to_string(&mut contents)
//             .map_err(|err| match err.kind() {
//                 // Permission denied
//                 io::ErrorKind::PermissionDenied => {
//                     Error::new(ErrorKind::PermissionDenied, "permission denied")
//                 }

//                 // Interrupted
//                 io::ErrorKind::Interrupted => Error::new(ErrorKind::Interrupted, "interrupted"),

//                 // Other error
//                 _ => Error::new(ErrorKind::Other, "other error"),
//             })?;

//         // Return the contents
//         Ok(contents)
//     }
// }

// #[cfg(test)]
// mod tests {
//     /*
//         This test assumes that there is a directory named 'temp' in the parent directory of './app'.
//         in other words, for './app', it would be '../temp'.

//         Command: e.g. cargo test --package fsh --bin fsh -- profile::tests::test_profile_new --exact --show-output
//                       cargo test --package fsh --bin fsh -- profile::tests --show-output
//     */
//     use super::*;
//     use std::fs;

//     #[test]
//     fn test_profile_new() {
//         let path = "../temp/test_profile_new";

//         // Remove the file if it exists.
//         let _ = fs::remove_file(path);

//         // Create a new profile.
//         let profile = Profile::new(path);

//         // Check if the profile was created successfully.
//         assert!(profile.is_ok());
//         assert!(exists(path));

//         // Remove the file.
//         let _ = fs::remove_file(path);
//     }

//     #[test]
//     fn test_profile_open() {
//         let path = "../temp/test_profile_open";

//         // Remove the file if it exists.
//         let _ = fs::remove_file(path);

//         // Open a profile that does not exist.
//         let profile = Profile::open(path);

//         // Check if the profile was opened successfully.
//         // is_err() returns true if the result is an error.
//         assert!(profile.is_err());
//         assert!(!exists(path));

//         // Create a new profile.
//         // Write some content to the file.
//         let _ = fs::write(path, "test");

//         // Open the profile.
//         let profile = Profile::open(path);

//         // Check if the profile was opened successfully.
//         assert!(profile.is_ok());
//         assert!(exists(path));

//         // Remove the file.
//         let _ = fs::remove_file(path);
//     }

//     #[test]
//     fn test_profile_write() {
//         let path = "../temp/test_profile_write";

//         // Remove the file if it exists.
//         let _ = fs::remove_file(path);

//         // Create a new profile.
//         let mut profile = Profile::new(path).unwrap();

//         // Write to the profile.
//         let result = profile.write("test");

//         // Check if the write operation was successful.
//         // Read the contents of the file and check if it matches the written content.
//         assert!(result.is_ok());
//         assert_eq!(fs::read_to_string(path).unwrap(), "test");

//         // Remove the file.
//         let _ = fs::remove_file(path);
//     }

//     #[test]
//     fn test_profile_read() {
//         let path = "../temp/test_profile_read";

//         // Remove the file if it exists.
//         let _ = fs::remove_file(path);

//         // Create a new profile.
//         // Write some content to the file.
//         let _ = fs::write(path, "test");

//         // Open the profile.
//         let mut profile = Profile::open(path).unwrap();

//         // Read the contents of the profile.
//         let result = profile.read();

//         // Check if the read operation was successful.
//         assert!(result.is_ok());
//         assert_eq!(result.unwrap(), "test");

//         // Remove the file.
//         let _ = fs::remove_file(path);
//     }

//     #[test]
//     fn test_exists() {
//         let path = "../temp/test_exists";

//         // Remove the file if it exists.
//         let _ = fs::remove_file(path);

//         // Check if the file exists.
//         assert!(!exists(path));

//         // Create a new file.
//         let _ = fs::write(path, "test");

//         // Check if the file exists.
//         assert!(exists(path));

//         // Remove the file.
//         let _ = fs::remove_file(path);
//     }
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
}
