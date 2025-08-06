use std::{borrow::Cow, env, path};

use crate::manifest;

const SHELL_NAME: &str = "\\s";

const SHELL_VERSION: &str = "\\v";

const HOST_NAME: &str = "\\h";

const USER_NAME: &str = "\\u";

const CURRENT_DIRECTORY: &str = "\\w";

const CURRENT_DIRECTORY_FULL: &str = "\\W";

pub const DEFAULT_PROMPT: &str = "\\u@\\w $ ";

#[inline]
fn get_shell_name() -> Cow<'static, str> {
    manifest::MANIFEST_FSH_NAME.into()
}

#[inline]
fn get_shell_version() -> Cow<'static, str> {
    manifest::MANIFEST_FSH_VERSION.into()
}

#[inline]
fn get_host_name() -> Cow<'static, str> {
    env::var("HOSTNAME").unwrap_or_default().into()
}

#[inline]
fn get_user_name() -> Cow<'static, str> {
    env::var("USER").unwrap_or_default().into()
}

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

#[inline]
fn get_current_directory_full() -> String {
    env::current_dir()
        .unwrap_or(path::PathBuf::from("./"))
        .to_string_lossy()
        .to_string()
}

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