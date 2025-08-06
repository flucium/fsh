use std::{env::home_dir, path::PathBuf};

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
}
