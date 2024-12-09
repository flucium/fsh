use crate::{error::Error, result::Result};
use std::os::unix::io::RawFd;

/// Represents the state of a pipe.
///
/// A pipe can be in one of three states:
/// - `Closed`: The pipe is not active.
/// - `Sendable`: The pipe is ready to send data.
/// - `Recvable`: The pipe is ready to receive data.
#[derive(PartialEq, Eq)]
pub enum PipeState {
    Closed = 0,

    Sendable = 1,

    Recvable = 2,
}

/// Manages the lifecycle and operations of a Unix pipe.
///
/// The `Pipe` struct provides a simplified abstraction for handling
/// a pipe's state and file descriptor. It supports opening, closing,
/// sending, and receiving operations.s
#[derive(PartialEq, Eq)]
pub struct Pipe {
    state: PipeState,
    fd: Option<RawFd>,
}

impl Pipe {
    /// Creates a new, closed `Pipe`.
    ///
    /// # Returns
    /// - A `Pipe` instance with `state` set to `Closed` and no file descriptor.
    pub fn new() -> Self {
        Self {
            state: PipeState::Closed,
            fd: None,
        }
    }

    /// Creates a new, sendable `Pipe`.
    ///
    /// # Returns
    /// - A `Pipe` instance with `state` set to `Sendable` and no file descriptor.
    pub fn open() -> Self {
        Self {
            state: PipeState::Sendable,
            fd: None,
        }
    }

    /// Retrieves the current state of the pipe.
    ///
    /// # Returns
    /// - A reference to the `PipeState` representing the pipe's state.
    pub fn state(&self) -> &PipeState {
        &self.state
    }

    /// Retrieves the file descriptor of the pipe, if available.
    ///
    /// # Returns
    /// - `Some(&RawFd)`: The file descriptor if it is set.
    /// - `None`: If the pipe does not have an associated file descriptor.
    pub fn fd(&self) -> Option<&RawFd> {
        self.fd.as_ref()
    }

    /// Closes the pipe and releases the file descriptor.
    ///
    /// # Returns
    /// - `Ok(())`: If the pipe was successfully closed.
    /// - `Err(Error)`: If the file descriptor is invalid or another error occurs.
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

    /// Quits the pipe by closing the file descriptor and resetting its state.
    ///
    /// This method is similar to `close` but does not return a result.
    pub fn quit(&mut self) {
        if let Some(fd) = self.fd.take() {
            unsafe { libc::close(fd) };
        }

        self.state = PipeState::Closed;
        self.fd = None;
    }

    /// Cancels the pipe, resetting it to the sendable state.
    ///
    /// This method closes the file descriptor and sets the state to `Sendable`.
    ///
    /// # Arguments
    /// - `self`: Consumes the current `Pipe` instance.
    pub fn cancel(mut self) {
        if let Some(fd) = self.fd {
            unsafe { libc::close(fd) };
        }

        self.state = PipeState::Sendable;
        self.fd = None;
    }

    /// Sends a file descriptor through the pipe.
    ///
    /// # Arguments
    /// - `fd`: The file descriptor to send.
    ///
    /// # Returns
    /// - `Ok(())`: If the file descriptor was successfully sent.
    /// - `Err(Error)`: If the pipe is not in the sendable state or another error occurs.
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

    /// Receives a file descriptor from the pipe.
    ///
    /// # Returns
    /// - `Ok(RawFd)`: The received file descriptor.
    /// - `Err(Error)`: If the pipe is not in the receivable state or another error occurs.
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
