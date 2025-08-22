use std::{
    collections::HashMap,
    env::{self, Vars},
};

use crate::{error::*, result::*};

/// Shell variables.
#[derive(Debug, Clone)]
pub struct ShVars(HashMap<String, String>);

impl ShVars {
    /// Creates an empty `ShVars`.
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Extends the variables with the given key-value pairs.
    pub fn inherit(&mut self, env_vars: Vars) {
        self.0.extend(env_vars);
    }

    /// Inserts a variable.
    ///
    /// Empty values are replaced with `"null"`.  
    /// An empty key returns an error.
    ///
    /// # Returns
    /// - `Ok(())` if the variable was inserted.  
    /// - `Err(Error)` if the key is empty.
    pub fn insert(&mut self, key: impl Into<String>, value: impl Into<String>) -> Result<()> {
        let key = key.into();

        let value = Some(value.into())
            .filter(|s: &String| !s.is_empty())
            .unwrap_or_else(|| "null".to_string());

        if key.is_empty() {
            Err(Error::new(ErrorKind::InvalidInput, "key must not be empty"))?
        }

        self.0.insert(key, value);

        Ok(())
    }

    /// Returns a reference to the value corresponding to the key.
    pub fn get(&self, key: impl Into<String>) -> Option<&String> {
        self.0.get(&key.into())
    }

    /// Returns a vector of all keys.
    pub fn keys(&self) -> Vec<&String> {
        self.0.keys().collect()
    }

    /// Returns a vector of all values.
    pub fn values(&self) -> Vec<&String> {
        self.0.values().collect()
    }

    /// Returns all entries as a `HashMap` of references.
    pub fn entries(&self) -> HashMap<&String, &String> {
        self.0.iter().collect()
    }

    /// Removes a variable by key.
    ///
    /// # Returns
    /// - `Some(value)` if the variable existed.  
    /// - `None` otherwise.
    pub fn remove(&mut self, key: impl Into<String>) -> Option<String> {
        self.0.remove(&key.into())
    }

    /// Returns `true` if the variable with the given key exists.
    pub fn exists(&mut self, key: impl Into<String>) -> bool {
        self.0.contains_key(&key.into())
    }

    /// Returns the length.
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl From<env::Vars> for ShVars {
    fn from(vars: env::Vars) -> Self {
        Self(vars.collect())
    }
}
