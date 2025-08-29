pub mod path {

    use crate::{error::*, result::*};

    use std::{
        env::home_dir,
        ffi::OsStr,
        path::{Path, PathBuf},
    };

    /// Extension trait for `PathBuf` providing convenience methods.
    pub trait PathBufExt {
        /// Expands a leading `~` in the path to the user's home directory.
        fn expand_tilde(&mut self);

        /// Returns an iterator over paths that match the glob pattern.
        fn glob(&self) -> Result<glob::Paths>;
    }

    impl PathBufExt for PathBuf {
        /// Expands a leading `~` in this path to the user's home directory.
        fn expand_tilde(&mut self) {
            let path = &self.as_path().to_string_lossy().to_string();

            self.clear();

            self.push(expand_tilde_to_home_dir(path));
        }

        /// Performs glob expansion on this path string.
        ///
        /// # Returns
        /// - `Ok(Paths)` with an iterator over matches.  
        /// - `Err(Error)` if the pattern is invalid.
        fn glob(&self) -> Result<glob::Paths> {
            glob::glob(&self.to_string_lossy())
                .map_err(|_| Error::new(ErrorKind::InvalidPath, "invalid glob pattern"))
        }
    }

    /// Replaces a leading `~` in the given string with the user's home directory.
    #[inline]
    fn expand_tilde_to_home_dir(p: &str) -> String {
        p.replace(
            '~',
            &home_dir()
                .unwrap_or(String::default().into())
                .to_string_lossy(),
        )
    }

    /// Resolves a `target` path against a given `current` directory and
    /// returns the canonical absolute path.
    ///
    /// Both `current` and `target` must exist and be directories.
    ///
    /// # Arguments
    /// - `current` - The base directory to resolve from.
    /// - `target` - The target directory to resolve.
    ///
    /// # Returns
    /// - `Ok(PathBuf)` with the canonical absolute path.  
    /// - `Err(Error)` if a path is invalid, does not exist, or is not a directory.
    pub fn resolve_relative<A: AsRef<OsStr> + ?Sized, B: AsRef<OsStr> + ?Sized>(
        current: &A,
        target: &B,
    ) -> Result<PathBuf> {
        //
        // current
        //

        let current = Path::new(current).canonicalize().map_err(|_| {
            Error::new(
                ErrorKind::InvalidPath,
                "failed to canonicalize current path",
            )
        })?;

        if !current.is_dir() {
            Err(Error::new(
                ErrorKind::NotADirectory,
                "current path is not a directory",
            ))?
        }

        if !current.exists() {
            Err(Error::new(
                ErrorKind::NotFound,
                "current path does not exist",
            ))?;
        }

        //
        // target
        //

        let target = Path::new(target);

        if !target.is_dir() {
            Err(Error::new(
                ErrorKind::NotADirectory,
                "target path is not a directory",
            ))?
        }

        if !target.exists() {
            Err(Error::new(
                ErrorKind::NotFound,
                "target path does not exist",
            ))?;
        }

        // join -> canonicalize
        let path = Path::new(&current)
            .join(Path::new(target))
            .canonicalize()
            .map_err(|_| {
                Error::new(ErrorKind::InvalidPath, "failed to canonicalize joined path")
            })?;

        Ok(path)
    }

    #[cfg(test)]
    mod tests {
        //
        // e.g. cargo test --package fsh --lib -- utils::path::tests --exact --show-output
        //      cargo test --package fsh --lib -- utils::path::tests::test_resolve_relative --show-output

        use super::*;

        #[test]
        fn test_expand_tilde_to_home_dir() {
            assert_eq!(expand_tilde_to_home_dir("~"), "root");
            assert_eq!(expand_tilde_to_home_dir("~/"), "root/");
            assert_eq!(expand_tilde_to_home_dir("~/repos"), "root/repos");
        }

        #[test]
        fn test_resolve_relative() {
            assert!(resolve_relative("./", "./src").is_ok());

            assert!(resolve_relative("./", "./aaa").is_err());

            assert_eq!(resolve_relative("./", "./src"), Ok("./src".into()));

            assert_ne!(resolve_relative("./", "./aaa"), Ok("./aaa".into()));
        }

        #[test]
        fn test_pathbuf_ext_expand_tilde() {
            let mut pathbuf = std::path::PathBuf::from("~/repos");

            pathbuf.expand_tilde();

            assert_eq!(pathbuf.to_str(), Some("/root/repos"));

            assert_ne!(pathbuf.to_str(), Some("/root/repos/"));
        }

        #[test]
        fn test_pathbuf_ext_glob() {
            assert!(std::path::PathBuf::from("./src/*.rs").glob().is_ok());

            assert!(std::path::PathBuf::from("./src/l*b.*s").glob().is_ok());

            assert!(
                std::path::PathBuf::from("./src/a*t/expression.rs")
                    .glob()
                    .unwrap()
                    .count()
                    == 1
            );

            assert!(std::path::PathBuf::from("./src/*.jpg").glob().is_ok());

            assert!(
                std::path::PathBuf::from("./src/*.jpg")
                    .glob()
                    .unwrap()
                    .count()
                    == 0
            );

            assert!(std::path::PathBuf::from("./src/***").glob().is_err());
        }
    }
}
