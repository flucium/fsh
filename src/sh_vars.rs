use std::{
    collections::HashMap,
    env::{self, Vars},
};

use crate::{error::*, result::*};

/// A wrapper around a `HashMap<String, String>` for managing shell variables.
///
/// `ShVars` provides a simple interface for inserting, removing, and querying
/// key-value pairs that represent shell environment or user-defined variables.
#[derive(Debug, Clone)]
pub struct ShVars(HashMap<String, String>);

impl ShVars {
    /// Creates a new, empty `ShVars` instance.
    ///
    /// # Returns
    /// A new `ShVars` with no entries.
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Extends the internal map with environment variables from the host process.
    ///
    /// # Arguments
    /// - `env_vars`: An iterator over environment variables, typically `std::env::vars()`.
    pub fn inherit(&mut self, env_vars: Vars) {
        self.0.extend(env_vars);
    }

    /// Inserts a key-value pair into the variable map.
    ///
    /// - If the `key` is an empty string, this function returns an error.
    /// - If the `value` is an empty string, it is automatically replaced with the literal string `"null"`.
    /// - If the key already exists, its value is overwritten.
    ///
    /// # Arguments
    /// - `key`: The variable name to insert. Must be non-empty.
    /// - `value`: The value to associate with the key. If empty, it will default to `"null"`.
    ///
    /// # Returns
    /// - `Ok(())` if the insertion succeeds.
    /// - `Err(Error::NOT_IMPLEMENTED)` if the key is empty.
    pub fn insert(&mut self, key: impl Into<String>, value: impl Into<String>) -> Result<()> {
        let key = Some(key.into())
            .filter(|k| !k.is_empty())
            .ok_or(Error::NOT_IMPLEMENTED)?;

        let value = Some(value.into())
            .filter(|s: &String| !s.is_empty())
            .unwrap_or_else(|| "null".to_string());

        if key.is_empty() {
            Err(Error::NOT_IMPLEMENTED)?
        }

        self.0.insert(key, value);

        Ok(())
    }

    /// Retrieves the value associated with the given key.
    ///
    /// # Arguments
    /// - `key`: The variable name to look up.
    ///
    /// # Returns
    /// - `Some(&String)` if the key exists.
    /// - `None` otherwise.
    pub fn get(&self, key: impl Into<String>) -> Option<&String> {
        self.0.get(&key.into())
    }

    /// Returns a list of all variable keys.
    ///
    /// # Returns
    /// A `Vec` of references to the keys in the map.
    pub fn keys(&self) -> Vec<&String> {
        self.0.keys().collect()
    }

    /// Returns a list of all variable values.
    ///
    /// # Returns
    /// A `Vec` of references to the values in the map.
    pub fn values(&self) -> Vec<&String> {
        self.0.values().collect()
    }

    /// Returns all key-value pairs as references.
    ///
    /// # Returns
    /// A `HashMap` of references to keys and values.
    pub fn entries(&self) -> HashMap<&String, &String> {
        self.0.iter().collect()
    }

    /// Removes a key-value pair from the map.
    ///
    /// # Arguments
    /// - `key`: The variable name to remove.
    ///
    /// # Returns
    /// - `Some(String)` if the key existed and was removed.
    /// - `None` otherwise.
    pub fn remove(&mut self, key: impl Into<String>) -> Option<String> {
        self.0.remove(&key.into())
    }

    /// Checks whether a given key exists in the variable map.
    ///
    /// # Arguments
    /// - `key`: The variable name to check.
    ///
    /// # Returns
    /// `true` if the key exists, `false` otherwise.
    pub fn exists(&mut self, key: impl Into<String>) -> bool {
        self.0.contains_key(&key.into())
    }

    /// Returns the number of key-value pairs stored in the map.
    ///
    /// # Returns
    /// The total number of variables.
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl From<env::Vars> for ShVars {
    fn from(vars: env::Vars) -> Self {
        Self(vars.collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shvars_new() {
        let shvars = ShVars::new();
        assert_eq!(shvars.len(), 0);
    }

    #[test]
    fn test_shvars_insert_and_get() {
        let mut shvars = ShVars::new();

        assert_eq!(shvars.insert("key1", "value1").is_ok(), true);

        assert_eq!(shvars.get("key1"), Some(&"value1".to_string()));

        assert_eq!(shvars.len(), 1);
    }

    #[test]
    fn test_shvars_remove() {
        let mut shvars = ShVars::new();

        assert_eq!(shvars.insert("key1", "value1").is_ok(), true);

        assert_eq!(shvars.remove("key1"), Some("value1".to_string()));

        assert_eq!(shvars.len(), 0);
    }

    #[test]
    fn test_shvars_exists() {
        let mut shvars = ShVars::new();

        assert_eq!(shvars.insert("key1", "value1").is_ok(), true);

        assert!(shvars.exists("key1"));

        assert!(!shvars.exists("key2"));
    }

    #[test]
    fn test_shvars_keys_and_values() {
        let mut shvars = ShVars::new();

        for i in 0..100 {
            assert_eq!(shvars.insert(format!("key{i}"), format!("value{i}")).is_ok(),true);
        }

        let keys = shvars.keys();
        let values = shvars.values();

        assert_eq!(keys.len(), 100);
        assert_eq!(values.len(), 100);
    }

    #[test]
    fn test_shvars_entries() {
        let mut shvars = ShVars::new();

        for i in 0..100 {
            assert_eq!(shvars.insert(format!("key{i}"), format!("value{i}")).is_ok(),true);
        }

        let entries = shvars.entries();

        assert_eq!(entries.len(), 100);

        assert_eq!(
            entries.get(&"key1".to_string()),
            Some(&"value1".to_string()).as_ref()
        );
    }
}
