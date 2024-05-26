use std::{env, path};

/// User name.
const USER_NAME: &str = "\\u";

// Shell name.
const SHELL_NAME: &str = "\\s";

// Shell version.
const SHELL_VERSION: &str = "\\v";

// Current directory name.
const CURRENT_DIRECTORY: &str = "\\w";

// Current directory full path.
const CURRENT_DIRECTORY_FULL: &str = "\\W";

// Host name.
const HOST_NAME: &str = "\\h";

// Default prompt.
pub const DEFAULT_PROMPT: &str = "\\u@\\w $ ";

/// Get user name from environment variable.
fn get_user_name() -> String {
    env::var("USER").unwrap_or_default()
}

/// Get current directory name from environment variable.
fn get_current_directory() -> String {
    env::current_dir()
        .unwrap_or(path::PathBuf::from("./"))
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string()
}

/// Get current directory full path from environment variable.
///
/// This function is not used in the code.
fn get_current_directory_full() -> String {
    env::current_dir()
        .unwrap_or(path::PathBuf::from("./"))
        .to_string_lossy()
        .to_string()
}

/// Get host name from environment variable.
fn get_host_name() -> String {
    env::var("HOSTNAME").unwrap_or_default()
}

/// Decode the prompt string.
fn decode(source: impl Into<String>) -> String {
    let mut string = String::from(source.into());

    if string.contains(USER_NAME) {
        string = string.replace(USER_NAME, &get_user_name());
    }

    if string.contains(SHELL_NAME) {
        string = string.replace(SHELL_NAME, "fsh");
    }

    if string.contains(SHELL_VERSION) {
        string = string.replace(SHELL_VERSION, "0.0.1");
    }

    if string.contains(CURRENT_DIRECTORY) {
        string = string.replace(CURRENT_DIRECTORY, &get_current_directory());
    }

    if string.contains(CURRENT_DIRECTORY_FULL) {
        string = string.replace(CURRENT_DIRECTORY_FULL, &get_current_directory_full());
    }

    if string.contains(HOST_NAME) {
        string = string.replace(HOST_NAME, &get_host_name());
    }

    string
}

/// Prompt the user with the default prompt.
pub fn prompt(source: impl Into<String>) -> String {
    decode(source.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_user_name() {
        assert_eq!(get_user_name(), env::var("USER").unwrap_or_default());
    }

    #[test]
    fn test_get_current_directory() {
        assert_eq!(
            get_current_directory(),
            env::current_dir()
                .unwrap()
                .file_name()
                .unwrap()
                .to_string_lossy()
        );
    }

    #[test]
    fn test_get_current_directory_full() {
        assert_eq!(
            get_current_directory_full(),
            env::current_dir().unwrap().to_string_lossy()
        );
    }

    #[test]
    fn test_get_host_name() {
        assert_eq!(get_host_name(), env::var("HOSTNAME").unwrap_or_default());
    }

    #[test]
    fn test_decode() {
        let prompt = "\\u@\\w $ ";
        assert_eq!(
            decode(prompt),
            format!("{}@{} $ ", get_user_name(), get_current_directory())
        );
    }
}
