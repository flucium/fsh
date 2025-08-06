use std::{borrow::Cow, env, path};

use crate::manifest;

/// Escape sequence for the shell name.
const SHELL_NAME: &str = "\\s";

/// Escape sequence for the shell version.
const SHELL_VERSION: &str = "\\v";

/// Escape sequence for the host name.
const HOST_NAME: &str = "\\h";

// Escape sequence for the user name.
const USER_NAME: &str = "\\u";

/// Escape sequence for the current directory name (e.g., `repos`).
const CURRENT_DIRECTORY: &str = "\\w";

/// Escape sequence for the full current directory path (e.g., `/home/username/repos`).
const CURRENT_DIRECTORY_FULL: &str = "\\W";

/// The default prompt format string used in FSH.
///
/// By default, this expands to `username@current_directory $ `.
pub const DEFAULT_PROMPT: &str = "\\u@\\w $ ";

/// Returns the shell name from the package manifest (`Cargo.toml`).
#[inline]
fn get_shell_name() -> Cow<'static, str> {
    manifest::MANIFEST_FSH_NAME.into()
}

/// Returns the shell version from the package manifest (`Cargo.toml`).
#[inline]
fn get_shell_version() -> Cow<'static, str> {
    manifest::MANIFEST_FSH_VERSION.into()
}

/// Returns the hostname from the `HOSTNAME` environment variable (from : std::env::var("HOSTNAME")).
/// Falls back to an empty string if unset.
#[inline]
fn get_host_name() -> Cow<'static, str> {
    env::var("HOSTNAME").unwrap_or_default().into()
}


/// Returns the username from the `USER` environment variable (from : std::env::var("USER")).
/// Falls back to an empty string if unset.
#[inline]
fn get_user_name() -> Cow<'static, str> {
    env::var("USER").unwrap_or_default().into()
}


/// Returns the name of the current working directory (not the full path).
///
/// Falls back to `"."` if the current directory cannot be resolved.
#[inline]
fn get_current_dir() -> Cow<'static, str> {
    env::current_dir()
        .unwrap_or(path::PathBuf::from("./"))
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string()
        .into()
}

/// Returns the full path of the current working directory as a string.
///
/// Falls back to `"./"` if the current directory cannot be resolved.
#[inline]
fn get_current_directory_full() -> String {
    env::current_dir()
        .unwrap_or(path::PathBuf::from("./"))
        .to_string_lossy()
        .to_string()
}

/// Replaces supported prompt escape sequences in the input string
/// with their corresponding runtime values.
///
/// The following escape sequences are supported:
/// - `\s` → shell name
/// - `\v` → shell version
/// - `\h` → host name
/// - `\u` → user name
/// - `\w` → current directory name
/// - `\W` → full current directory path
///
/// # Arguments
/// - `source`: A string containing zero or more escape sequences.
///
/// # Returns
/// A `Cow<'static, str>` with all recognized escape sequences replaced.
pub fn decode(source: impl Into<String>) -> Cow<'static, str> {
    let mut source = source.into();

    if source.contains(SHELL_NAME) {
        source = source.replace(SHELL_NAME, &get_shell_name());
    }

    if source.contains(SHELL_VERSION) {
        source = source.replace(SHELL_VERSION, &get_shell_version());
    }

    if source.contains(HOST_NAME) {
        source = source.replace(HOST_NAME, &get_host_name());
    }

    if source.contains(USER_NAME) {
        source = source.replace(USER_NAME, &get_user_name());
    }

    if source.contains(CURRENT_DIRECTORY) {
        source = source.replace(CURRENT_DIRECTORY, &get_current_dir());
    }

    if source.contains(CURRENT_DIRECTORY_FULL) {
        source = source.replace(CURRENT_DIRECTORY_FULL, &get_current_directory_full());
    }

    source.into()
}