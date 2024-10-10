use crate::{error::Error, result::Result};
use std::os::unix::io::RawFd;


#[derive(PartialEq, Eq)]
pub enum PipeState {
    Closed = 0,

    Sendable = 1,

    Recvable = 2,
}


#[derive(PartialEq, Eq)]
pub struct Pipe {
    state: PipeState,
    fd: Option<RawFd>,
}

impl Pipe {
    pub fn new() -> Self {
        Self {
            state: PipeState::Closed,
            fd: None,
        }
    }

    pub fn open() -> Self {
        Self {
            state: PipeState::Sendable,
            fd: None,
        }
    }

    
    pub fn state(&self) -> &PipeState {
        &self.state
    }

    
    pub fn fd(&self) -> Option<&RawFd> {
        self.fd.as_ref()
    }

    
    pub fn close(&mut self) -> Result<()> {
        if let Some(fd) = self.fd {
            if fd >= 0 {
                unsafe { libc::close(fd) };
            } else {
                Err(Error::NOT_IMPLEMENTED)?
            }
        }

        self.state = PipeState::Closed;
        self.fd = None;
        Ok(())
    }

    
    pub fn quit(&mut self) {
        if let Some(fd) = self.fd.take() {
            unsafe { libc::close(fd) };
        }

        self.state = PipeState::Closed;
        self.fd = None;
    }

    
    pub fn cancel(mut self) {
        if let Some(fd) = self.fd {
            unsafe { libc::close(fd) };
        }

        self.state = PipeState::Sendable;
        self.fd = None;
    }

    
    pub fn send(&mut self, fd: RawFd) -> Result<()> {
        if self.fd.is_some() {
            Err(Error::NOT_IMPLEMENTED)?
        }

        match self.state {
            PipeState::Sendable => {
                self.fd = Some(fd);
                self.state = PipeState::Recvable;
                Ok(())
            }

            PipeState::Closed => Err(Error::NOT_IMPLEMENTED),

            PipeState::Recvable => Err(Error::NOT_IMPLEMENTED),
        }
    }

    pub fn recv(&mut self) -> Result<RawFd> {
        if self.fd.is_none() {
            Err(Error::NOT_IMPLEMENTED)?
        }

        match self.state {
            PipeState::Recvable => {
                let fd = self.fd.unwrap_or(-1);

                if fd < 0 {
                    Err(Error::NOT_IMPLEMENTED)?
                }

                self.state = PipeState::Sendable;
                self.fd = None;

                Ok(fd)
            }

            PipeState::Closed => Err(Error::NOT_IMPLEMENTED)?,

            PipeState::Sendable => Err(Error::NOT_IMPLEMENTED)?,
        }
    }
}
