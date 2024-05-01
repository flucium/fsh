use std::path::{Path, PathBuf};

use super::{pipe::*, process_handler::*};

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
            current_dir:PathBuf::new(),
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
