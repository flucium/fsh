use crate::{pipe::Pipe, process_handler::ProcessHandler};
use std::path::{Path, PathBuf};

/// Represents the current state of the shell or interpreter.
///
/// The `State` struct manages the following components:
/// - Process handling via `ProcessHandler`.
/// - Pipe management via `Pipe`.
/// - Tracking of the current working directory.
// -----
// # Fields
// - `handler`: Manages processes in the current state.
// - `pipe`: Manages inter-process communication pipes.
// - `current_dir`: Tracks the current working directory as a `PathBuf`.
pub struct State {
    handler: ProcessHandler,
    pipe: Pipe,
    current_dir: PathBuf,
}

impl State {
    /// Creates a new `State` with default values.
    pub fn new() -> Self {
        Self {
            handler: ProcessHandler::new(),
            pipe: Pipe::new(),
            current_dir: PathBuf::new(),
        }
    }

    /// Provides an immutable reference to the `ProcessHandler`.
    ///
    /// # Returns
    /// - A reference to the `ProcessHandler` for managing processes.
    pub fn handler(&self) -> &ProcessHandler {
        &self.handler
    }

    /// Provides a mutable reference to the `ProcessHandler`.
    ///
    /// # Returns
    /// - A mutable reference to the `ProcessHandler` for managing processes.
    pub fn handler_mut(&mut self) -> &mut ProcessHandler {
        &mut self.handler
    }

    /// Provides an immutable reference to the `Pipe`.
    ///
    /// # Returns
    /// - A reference to the `Pipe` for inter-process communication.
    pub fn pipe(&self) -> &Pipe {
        &self.pipe
    }

    /// Provides a mutable reference to the `Pipe`.
    ///
    /// # Returns
    /// - A mutable reference to the `Pipe` for inter-process communication.
    pub fn pipe_mut(&mut self) -> &mut Pipe {
        &mut self.pipe
    }

    /// Provides an immutable reference to the current working directory.
    ///
    /// # Returns
    /// - A reference to the `Path` representing the current working directory.
    pub fn current_dir(&self) -> &Path {
        &self.current_dir
    }

    /// Provides a mutable reference to the current working directory.
    ///
    /// # Returns
    /// - A mutable reference to the `PathBuf` representing the current working directory.
    pub fn current_dir_mut(&mut self) -> &mut PathBuf {
        &mut self.current_dir
    }
}

impl From<PathBuf> for State {
    fn from(path: PathBuf) -> Self {
        let mut state = State::new();
        *state.current_dir_mut() = path;
        state
    }
}

impl From<&Path> for State {
    fn from(path: &Path) -> Self {
        let mut state = State::new();
        *state.current_dir_mut() = path.to_path_buf();
        state
    }
}

impl From<String> for State {
    fn from(path: String) -> Self {
        let mut state = State::new();
        *state.current_dir_mut() = PathBuf::from(path);
        state
    }
}

impl From<&str> for State {
    fn from(path: &str) -> Self {
        let mut state = State::new();
        *state.current_dir_mut() = PathBuf::from(path);
        state
    }
}
