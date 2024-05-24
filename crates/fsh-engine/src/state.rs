use super::{pipe::*, process_handler::*};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct State {
    handler: ProcessHandler,
    pipe: Pipe,
    current_dir: PathBuf,
}

impl State {
    /// Creates a new state.
    pub fn new() -> Self {
        Self {
            handler: ProcessHandler::new(),
            pipe: Pipe::new(),
            current_dir: PathBuf::new(),
        }
    }

    /// Get the handler.
    pub fn handler(&self) -> &ProcessHandler {
        &self.handler
    }

    /// Get the mutable handler.
    pub fn handler_mut(&mut self) -> &mut ProcessHandler {
        &mut self.handler
    }

    /// Get the pipe.
    pub fn pipe(&self) -> &Pipe {
        &self.pipe
    }

    /// Get the mutable pipe.
    pub fn pipe_mut(&mut self) -> &mut Pipe {
        &mut self.pipe
    }

    /// Get the current directory.
    pub fn current_dir(&self) -> &Path {
        &self.current_dir
    }

    /// Get the mutable current directory.
    pub fn current_dir_mut(&mut self) -> &mut PathBuf {
        &mut self.current_dir
    }
}

impl From<PathBuf> for State {
    /// Creates a new state with the given path.
    ///
    /// # Arguments
    /// - `path` - The path to set as the current directory.
    fn from(path: PathBuf) -> Self {
        let mut state = State::new();
        *state.current_dir_mut() = path;
        state
    }
}

impl From<&Path> for State {
    /// Creates a new state with the given path.
    ///
    /// # Arguments
    /// - `path` - The path to set as the current directory.
    fn from(path: &Path) -> Self {
        let mut state = State::new();
        *state.current_dir_mut() = path.to_path_buf();
        state
    }
}

impl From<String> for State {
    /// Creates a new state with the given path.
    ///
    /// # Arguments
    /// - `path` - The path to set as the current directory.
    fn from(path: String) -> Self {
        let mut state = State::new();
        *state.current_dir_mut() = PathBuf::from(path);
        state
    }
}

impl From<&str> for State {
    /// Creates a new state with the given path.
    ///
    /// # Arguments
    /// - `path` - The path to set as the current directory.
    fn from(path: &str) -> Self {
        let mut state = State::new();
        *state.current_dir_mut() = PathBuf::from(path);
        state
    }
}
