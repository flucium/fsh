use std::{
    io::{PipeReader, PipeWriter},
    path::{Path, PathBuf},
    process,
};

/// Represents the global state of the shell during execution.
///
/// This includes child processes, active pipe handles,
/// and the current working directory context.
pub struct State {
    /// A list of spawned child processes and a flag indicating foreground/background execution.
    processes: Vec<(process::Child, bool)>,

    /// A tuple representing the read and write ends of the active pipe, if any.
    pipe: (Option<PipeReader>, Option<PipeWriter>),

    /// The current working directory.
    current_dir: PathBuf,
}

impl State {
    /// Creates a new `State` with no processes, no pipe, and an empty working directory.
    pub fn new() -> Self {
        Self {
            processes: Vec::new(),
            pipe: (None, None),
            current_dir: PathBuf::new(),
        }
    }

    /// Returns an immutable reference to the list of child processes.
    ///
    /// Each element is a tuple of `(Child, is_foreground or is_background)`.
    pub fn processes(&self) -> &Vec<(process::Child, bool)> {
        &self.processes
    }

    /// Returns a mutable reference to the list of child processes.
    pub fn processes_mut(&mut self) -> &mut Vec<(process::Child, bool)> {
        &mut self.processes
    }

    /// Returns an immutable reference to the current pipe endpoints.
    ///
    /// The pipe is represented as a pair of `Option` values: reader and writer.
    pub fn pipe(&self) -> &(Option<PipeReader>, Option<PipeWriter>) {
        &self.pipe
    }

    /// Returns a mutable reference to the current pipe endpoints.
    pub fn pipe_mut(&mut self) -> &mut (Option<PipeReader>, Option<PipeWriter>) {
        &mut self.pipe
    }

    /// Returns an immutable reference to the current working directory.
    pub fn current_dir(&self) -> &Path {
        &self.current_dir
    }

    /// Returns a mutable reference to the current working directory.
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
