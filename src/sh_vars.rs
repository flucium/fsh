use std::{
    collections::HashMap,
    env::{self, Vars},
};

#[derive(Debug, Clone)]
pub struct ShVars(HashMap<String, String>);

impl ShVars {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn inherit(&mut self, env_vars: Vars) {
        self.0.extend(env_vars);
    }

    pub fn insert(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.0.insert(key.into(), value.into());
    }

    pub fn get(&self, key: impl Into<String>) -> Option<&String> {
        self.0.get(&key.into())
    }

    pub fn keys(&self) -> Vec<&String> {
        self.0.keys().collect()
    }

    pub fn values(&self) -> Vec<&String> {
        self.0.values().collect()
    }

    pub fn entries(&self) -> HashMap<&String, &String> {
        self.0.iter().collect()
    }

    pub fn remove(&mut self, key: impl Into<String>) -> Option<String> {
        self.0.remove(&key.into())
    }

    pub fn exists(&mut self, key: impl Into<String>) -> bool {
        self.0.contains_key(&key.into())
    }
}

impl From<env::Vars> for ShVars {
    fn from(vars: env::Vars) -> Self {
        Self(vars.collect())
    }
}
