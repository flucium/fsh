use fsh_common::{Error, ErrorKind, Result};
use std::{collections::HashMap, fs, io::Write, path::Path};

pub const RESERVEDWORD_SHELL_VARIABLE_FSH_PROMPT: &str = "FSH_PROMPT";

pub const RESERVEDWORD_SHELL_VARIABLE_FSH_CWD: &str = "FSH_CWD";

/// Shell variables.
#[derive(Debug, Clone)]
pub struct ShVars(HashMap<String, String>);

impl ShVars {
    /// Create a new instance of `ShVars`.
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Inherit environment variables.
    pub fn inherit(&mut self, env_vars: std::env::Vars) -> &mut Self {
        for (key, value) in env_vars {
            self.0.insert(key, value);
        }

        self
    }

    /// Open a shell variables file.
    pub fn open(path: &Path) -> Result<Self> {
        let mut vars = HashMap::new();

        let file = match fs::read_to_string(path) {
            Ok(file) => file,
            Err(err) => {
                if err.kind() == std::io::ErrorKind::NotFound {
                    Err(Error::new(
                        ErrorKind::NotFound,
                        "shell variables file not found",
                    ))?
                } else {
                    Err(Error::new(ErrorKind::Other, &err.to_string()))?
                }
            }
        };

        for line in file.lines() {
            if line.is_empty() {
                continue;
            }

            let mut parts = line.split('=');

            let key = match parts.next() {
                Some(key) => key,
                None => continue,
            };

            let value = match parts.next() {
                Some(value) => value,
                None => continue,
            };

            vars.insert(
                key.trim_start().trim_end().to_string(),
                value.trim_start().trim_end().to_string(),
            );
        }
        Ok(Self(vars))
    }

    /// Save shell variables to a file.
    pub fn save(&self, path: &Path) -> Result<()> {
        let mut file = match fs::File::options().create(true).write(true).open(path) {
            Ok(file) => file,
            Err(err) => {
                if err.kind() == std::io::ErrorKind::PermissionDenied {
                    Err(Error::new(ErrorKind::PermissionDenied, "permission denied"))?
                } else {
                    Err(Error::new(ErrorKind::Other, &err.to_string()))?
                }
            }
        };

        for (key, value) in &self.0 {
            let line = format!("{}={}\n", key, value);

            file.write_all(line.as_bytes()).map_err(|err| {
                if err.kind() == std::io::ErrorKind::Interrupted {
                    Error::new(
                        ErrorKind::Interrupted,
                        "the operation was interrupted before it could be completed",
                    )
                } else {
                    Error::new(ErrorKind::Other, &err.to_string())
                }
            })?;

            file.write(b"\n").map_err(|err| {
                if err.kind() == std::io::ErrorKind::Interrupted {
                    Error::new(
                        ErrorKind::Interrupted,
                        "the operation was interrupted before it could be completed",
                    )
                } else {
                    Error::new(ErrorKind::Other, &err.to_string())
                }
            })?;
        }

        Ok(())
    }

    /// Get a shell variable by key.
    pub fn get(&self, key: &str) -> Result<&str> {
        self.0.get(key).map(|value| value.as_str()).ok_or_else(|| {
            Error::new(
                ErrorKind::NotFound,
                &format!("the key '{}' does not exist", key),
            )
        })
    }

    /// Get the prompt.
    pub fn get_prompt(&self) -> Result<&str> {
        self.get(RESERVEDWORD_SHELL_VARIABLE_FSH_PROMPT)
    }

    /// Get the current working directory.
    pub fn get_cwd(&self) -> Result<&str> {
        self.get(RESERVEDWORD_SHELL_VARIABLE_FSH_CWD)
    }

    /// Set a shell variable.
    pub fn insert(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.0.insert(key.into(), value.into());
    }

    /// Unset a shell variable.
    pub fn remove(&mut self, key: &str) {
        self.0.remove(key);
    }

    /// Check if a shell variable exists.
    pub fn exists(&self, key: &str) -> bool {
        self.0.contains_key(key)
    }

    /// Get the length of shell variables.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Check if shell variables are empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Clear shell variables.
    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// Get all keys of shell variables.
    pub fn keys(&self) -> Vec<&String> {
        self.0.keys().collect()
    }

    /// Get all values of shell variables.
    pub fn values(&self) -> Vec<&String> {
        self.0.values().collect()
    }

    /// Get all key-value pairs of shell variables.
    pub fn entries(&self) -> HashMap<&String, &String> {
        self.0.iter().collect()
    }
}

impl From<HashMap<String, String>> for ShVars {
    /// Convert a `HashMap<String, String>` to `ShVars`.
    ///
    /// # Arguments
    /// - `vars` - The `HashMap` to convert.
    fn from(vars: HashMap<String, String>) -> Self {
        Self(vars)
    }
}

impl From<HashMap<&str, &str>> for ShVars {
    /// Convert a `HashMap<&str, &str>` to `ShVars`.
    ///
    /// # Arguments
    /// - `vars` - The `HashMap` to convert.
    fn from(vars: HashMap<&str, &str>) -> Self {
        let mut map = HashMap::new();
        for (key, value) in vars {
            map.insert(key.to_string(), value.to_string());
        }
        Self(map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sh_vars() {
        let vars = ShVars::new();

        assert_eq!(vars.len(), 0);

        assert_eq!(vars.is_empty(), true);
    }

    #[test]
    fn test_sh_vars_insert() {
        let mut vars = ShVars::new();

        for i in 0..10 {
            vars.insert(format!("key{}", i), format!("value{}", i));
        }

        for i in 0..10 {
            assert_eq!(
                vars.get(&format!("key{}", i)).unwrap(),
                &format!("value{}", i)
            );
        }

        assert_eq!(vars.len(), 10);

        assert_eq!(vars.is_empty(), false);
    }

    #[test]
    fn test_sh_vars_get() {
        let vars = ShVars::from(HashMap::from([
            ("key1".to_string(), "value1".to_string()),
            ("key2".to_string(), "value2".to_string()),
            ("key3".to_string(), "value3".to_string()),
        ]));

        for i in 1..4 {
            assert_eq!(
                vars.get(&format!("key{}", i)).unwrap(),
                &format!("value{}", i)
            );
        }

        assert_eq!(vars.len(), 3);

        assert_eq!(vars.is_empty(), false);
    }

    #[test]
    fn test_sh_vars_remove() {
        let mut vars = ShVars::from(HashMap::from([
            ("key1".to_string(), "value1".to_string()),
            ("key2".to_string(), "value2".to_string()),
            ("key3".to_string(), "value3".to_string()),
        ]));

        vars.remove("key1");

        assert_eq!(vars.exists("key1"), false);

        assert_eq!(vars.len(), 2);
    }

    #[test]
    fn test_sh_vars_clear() {
        let mut vars = ShVars::from(HashMap::from([
            ("key1".to_string(), "value1".to_string()),
            ("key2".to_string(), "value2".to_string()),
            ("key3".to_string(), "value3".to_string()),
        ]));

        vars.clear();

        assert_eq!(vars.len(), 0);

        assert_eq!(vars.is_empty(), true);
    }
}
