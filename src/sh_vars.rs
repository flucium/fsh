use std::{
    collections::HashMap,
    env::{self, Vars},
};

/// Represents a collection of shell variables.
///
/// The `ShVars` struct is a wrapper around a `HashMap` that stores shell variables as
/// key-value pairs of `String`. It provides methods for managing shell variables,
/// including insertion, retrieval, removal, and inheriting environment variables.
#[derive(Debug, Clone)]
pub struct ShVars(HashMap<String, String>);

impl ShVars {
    /// Creates a new, empty `ShVars` instance.
    ///
    /// # Returns
    /// - A new `ShVars` instance with no variables.
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Inherits environment variables into the `ShVars` instance.
    ///
    /// This method adds all environment variables from the provided `std::env::Vars` iterator
    /// to the current shell variable collection.
    ///
    /// # Arguments
    /// - `env_vars`: An iterator over the environment variables to inherit.
    pub fn inherit(&mut self, env_vars: Vars) {
        self.0.extend(env_vars);
    }

    /// Inserts a key-value pair into the shell variables.
    ///
    /// If the key already exists, its value is updated with the new value.
    ///
    /// # Arguments
    /// - `key`: The variable name to insert.
    /// - `value`: The variable value to associate with the key.s
    pub fn insert(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.0.insert(key.into(), value.into());
    }

    /// Retrieves the value associated with the specified key.
    ///
    /// # Arguments
    /// - `key`: The variable name to look up.
    ///
    /// # Returns
    /// - `Some(&String)`: The value associated with the key, if it exists.
    /// - `None`: If the key is not present.
    pub fn get(&self, key: impl Into<String>) -> Option<&String> {
        self.0.get(&key.into())
    }

    /// Returns a vector of all variable names in the collection.
    ///
    /// # Returns
    /// - A vector of references to the variable names.
    pub fn keys(&self) -> Vec<&String> {
        self.0.keys().collect()
    }

    /// Returns a vector of all variable values in the collection.
    ///
    /// # Returns
    /// - A vector of references to the variable values.
    pub fn values(&self) -> Vec<&String> {
        self.0.values().collect()
    }

    /// Returns a `HashMap` of all key-value pairs in the collection.
    ///
    /// # Returns
    /// - A `HashMap` with references to the keys and values.
    pub fn entries(&self) -> HashMap<&String, &String> {
        self.0.iter().collect()
    }

    /// Removes a variable by its key.
    ///
    /// If the key exists, the variable is removed, and its value is returned.
    ///
    /// # Arguments
    /// - `key`: The variable name to remove.
    ///
    /// # Returns
    /// - `Some(String)`: The value of the removed variable, if it existed.
    /// - `None`: If the key did not exist.
    pub fn remove(&mut self, key: impl Into<String>) -> Option<String> {
        self.0.remove(&key.into())
    }

    /// Checks if a variable exists in the collection.
    ///
    /// # Arguments
    /// - `key`: The variable name to check.
    ///
    /// # Returns
    /// - `true`: If the variable exists.
    /// - `false`: If the variable does not exist.
    pub fn exists(&mut self, key: impl Into<String>) -> bool {
        self.0.contains_key(&key.into())
    }
}

impl From<env::Vars> for ShVars {
    /// Creates a `ShVars` instance from environment variables.
    ///
    /// This method converts an iterator of environment variables (`Vars`) into a
    /// `ShVars` instance containing the same key-value pairs.
    ///
    /// # Arguments
    /// - `vars`: An iterator over environment variables.
    ///
    /// # Returns
    /// - A `ShVars` instance initialized with the environment variables.
    fn from(vars: env::Vars) -> Self {
        Self(vars.collect())
    }
}
