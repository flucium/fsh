use crate::{pipe::Pipe, process_handler::ProcessHandler};
use std::path::{Path, PathBuf};

pub struct State {
    handler: ProcessHandler,
    pipe: Pipe,
    current_dir: PathBuf,
}

impl State {
    pub fn new() -> Self {
        Self {
            handler: ProcessHandler::new(),
            pipe: Pipe::new(),
            current_dir: PathBuf::new(),
        }
    }

    pub fn handler(&self) -> &ProcessHandler {
        &self.handler
    }

    pub fn handler_mut(&mut self) -> &mut ProcessHandler {
        &mut self.handler
    }

    pub fn pipe(&self) -> &Pipe {
        &self.pipe
    }

    pub fn pipe_mut(&mut self) -> &mut Pipe {
        &mut self.pipe
    }

    pub fn current_dir(&self) -> &Path {
        &self.current_dir
    }

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
