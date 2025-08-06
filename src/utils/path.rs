use crate::{error::*, result::*};

use std::{
    env::home_dir,
    ffi::OsStr,
    path::{Path, PathBuf},
};

/// Resolves a target path relative to a given current path, returning the canonical absolute path.
///
/// This function first canonicalizes the `current` path, ensuring it is an existing directory.
/// It then joins the `target` path to the `current` path and canonicalizes the result,
/// returning the resolved absolute `PathBuf`.
///
/// Both paths are interpreted as filesystem paths and resolved via `std::fs::canonicalize`.
///
/// # Arguments
/// - `current`: The base directory from which to resolve the relative path.
/// - `target`: The path to resolve relative to `current`.
///
/// # Returns
/// - `Ok(PathBuf)` if the path is resolved successfully.
/// - `Err(Error::NOT_IMPLEMENTED)` if any filesystem operation fails.
pub fn resolve_relative<A: AsRef<OsStr> + ?Sized, B: AsRef<OsStr> + ?Sized>(
    current: &A,
    target: &B,
) -> Result<PathBuf> {
    // current path: Path::new(current) -> canonicalize
    let current = Path::new(current)
        .canonicalize()
        .map_err(|_| Error::NOT_IMPLEMENTED)?;

    // target path: Path::new(target)
    let target = Path::new(target);

    if !current.is_dir() {
        Err(Error::NOT_IMPLEMENTED)?
    }

    // current <- join <- target <- canonicalize
    let path = Path::new(&current)
        .join(Path::new(target))
        .canonicalize()
        .map_err(|_| Error::NOT_IMPLEMENTED)?;

    if !current.is_dir() {
        Err(Error::NOT_IMPLEMENTED)?
    }

    Ok(path)
}

/// Expands a leading tilde (`~`) in the given path to the user's home directory.
///
/// This function replaces all instances of `~` in the input string with the user's home directory
/// path, as determined by the `home_dir()` function. The result is returned as a `PathBuf`.
///
/// If the home directory cannot be determined, an empty path is substituted instead.
pub fn expand_tilde_to_home_dir(path: impl Into<String>) -> PathBuf {
    path.into()
        .replace(
            '~',
            &home_dir()
                .unwrap_or(String::default().into())
                .to_string_lossy(),
        )
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_tilde_to_home_dir() {
        // const PATH: &str = "/Users/<you-name>";
        const PATH: &str = "/Users/flucium";

        assert_eq!(
            expand_tilde_to_home_dir("~"),
            PathBuf::from(format!("{PATH}"))
        );

        assert_eq!(
            expand_tilde_to_home_dir("~/"),
            PathBuf::from(format!("{PATH}"))
        );

        assert_eq!(
            expand_tilde_to_home_dir("~"),
            PathBuf::from(format!("{PATH}/"))
        );

        assert_eq!(
            expand_tilde_to_home_dir("~/"),
            PathBuf::from(format!("{PATH}/"))
        );

        assert_eq!(
            expand_tilde_to_home_dir("~///"),
            PathBuf::from(format!("{PATH}"))
        );

        assert_eq!(
            expand_tilde_to_home_dir("~"),
            PathBuf::from(format!("{PATH}///"))
        );

        assert_eq!(
            expand_tilde_to_home_dir("~///"),
            PathBuf::from(format!("{PATH}///"))
        );
    }

    #[test]
    fn test_resolve_relative() {
        assert_eq!(resolve_relative("../../", "../").is_ok(), true);

        assert_eq!(resolve_relative("../../", "../../").is_ok(), true);

        assert_eq!(resolve_relative("../", "../../").is_ok(), true);

        assert_eq!(resolve_relative("../", "").is_ok(), true);

        assert_eq!(resolve_relative("/", "../../").is_ok(), true);

        assert_eq!(resolve_relative("///", "///").is_ok(), true);

        // assert_eq!(
        //     resolve_relative("/Users/flucium/repos/", "/Users/flucium/")
        //         .unwrap()
        //         .to_string_lossy(),
        //     "/Users/flucium"
        // );
    }

    #[test]
    fn test_resolve_relative_failure() {
        assert_eq!(resolve_relative("", "").is_err(), true);
    }
}
