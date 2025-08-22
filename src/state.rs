use std::{
    io::{PipeReader, PipeWriter},
    path::{Path, PathBuf},
    process,
};

/// Shell state.
///
/// This includes child processes, active pipe handles,
/// and the current working directory context.
pub struct State {
    processes: Vec<(process::Child, bool)>,

    pipe: (Option<PipeReader>, Option<PipeWriter>),

    current_dir: PathBuf,
}

impl State {
    /// Creates a new empty `State`.
    pub fn new() -> Self {
        Self {
            processes: Vec::new(),
            pipe: (None, None),
            current_dir: PathBuf::new(),
        }
    }

    /// Returns an immutable reference to the list of processes.
    pub fn processes(&self) -> &Vec<(process::Child, bool)> {
        &self.processes
    }

    /// Returns a mutable reference to the list of processes.
    pub fn processes_mut(&mut self) -> &mut Vec<(process::Child, bool)> {
        &mut self.processes
    }

    /// Returns an immutable reference to the current pipe.
    pub fn pipe(&self) -> &(Option<PipeReader>, Option<PipeWriter>) {
        &self.pipe
    }

    /// Returns a mutable reference to the current pipe.
    pub fn pipe_mut(&mut self) -> &mut (Option<PipeReader>, Option<PipeWriter>) {
        &mut self.pipe
    }

    /// Returns the current working directory.
    pub fn current_dir(&self) -> &Path {
        &self.current_dir
    }

    /// Returns a mutable reference to the current working directory.
    pub fn current_dir_mut(&mut self) -> &mut PathBuf {
        &mut self.current_dir
    }
}
