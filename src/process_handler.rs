use crate::{
    error::{Error, ErrorKind},
    result::Result,
};
use std::{mem::ManuallyDrop, process};

/// Manages a collection of processes for handling foreground and background tasks.
///
/// The `ProcessHandler` struct provides utilities for managing process lifecycle,
/// including starting, stopping, and tracking processes. It supports both foreground
/// and background execution.
pub struct ProcessHandler(Vec<(ManuallyDrop<process::Child>, bool)>);

impl ProcessHandler {
    /// Creates a new, empty `ProcessHandler`.
    ///
    /// # Returns
    /// - A new `ProcessHandler` instance with an empty process list.
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Creates a new `ProcessHandler` with pre-allocated capacity.
    ///
    /// # Arguments
    /// - `capacity`: The number of processes to pre-allocate storage for.
    ///
    /// # Returns
    /// - A new `ProcessHandler` instance with the specified capacity.
    pub fn new_with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    /// Adds a new process to the handler.
    ///
    /// # Arguments
    /// - `ps`: The process to add as a `process::Child`.
    /// - `is_background`: A boolean indicating if the process is running in the background.
    ///
    /// # Returns
    /// - The process ID (`pid`) of the added process.
    pub fn push(&mut self, ps: process::Child, is_background: bool) -> u32 {
        let pid = ps.id();

        self.0.push((ManuallyDrop::new(ps), is_background));

        pid
    }

    /// Removes and returns the last process from the handler.
    ///
    /// # Returns
    /// - `Some(process::Child)`: The removed process if the list is not empty.
    /// - `None`: If the process list is empty.
    pub fn pop(&mut self) -> Option<std::process::Child> {
        self.0.pop().map(|(ps, _)| ManuallyDrop::into_inner(ps))
    }

    /// Retrieves a reference to a process by its process ID (`pid`).
    ///
    /// # Arguments
    /// - `pid`: The process ID to look up.
    ///
    /// # Returns
    /// - `Some(&process::Child)`: A reference to the process if found.
    /// - `None`: If no process with the given ID exists.
    pub fn get(&self, pid: u32) -> Option<&std::process::Child> {
        self.0
            .iter()
            .find(|(ps, _)| ps.id() == pid)
            .map(|(ps, _)| &**ps)
    }

    /// Retrieves a mutable reference to a process by its process ID (`pid`).
    ///
    /// # Arguments
    /// - `pid`: The process ID to look up.
    ///
    /// # Returns
    /// - `Some(&mut process::Child)`: A mutable reference to the process if found.
    /// - `None`: If no process with the given ID exists.
    pub fn get_mut(&mut self, pid: u32) -> Option<&mut std::process::Child> {
        self.0
            .iter_mut()
            .find(|(ps, _)| ps.id() == pid)
            .map(|(ps, _)| &mut **ps)
    }

    /// Removes a process by its process ID (`pid`).
    ///
    /// # Arguments
    /// - `pid`: The process ID of the process to remove.
    ///
    /// # Returns
    /// - `Ok(())`: If the process was successfully removed.
    /// - `Err(Error)`: If no process with the given ID exists.
    pub fn remove(&mut self, pid: u32) -> Result<()> {
        let index = self
            .0
            .iter()
            .position(|(ps, _)| ps.id() == pid)
            .ok_or_else(|| Error::new(ErrorKind::NotFound, ""))?;

        unsafe {
            ManuallyDrop::drop(&mut self.0.remove(index).0);
        }

        Ok(())
    }

    /// Retrieves references to all managed processes.
    ///
    /// # Returns
    /// - A vector of references to all processes.
    pub fn entries(&self) -> Vec<&std::process::Child> {
        self.0.iter().map(|(ps, _)| &**ps).collect()
    }

    /// Returns the number of processes currently managed.
    ///
    /// # Returns
    /// - The number of processes in the handler.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns the capacity of the internal process list.
    ///
    /// # Returns
    /// - The capacity of the process list.
    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    /// Kills a process by its process ID (`pid`).
    ///
    /// # Arguments
    /// - `pid`: The process ID of the process to kill.
    ///
    /// # Returns
    /// - `Ok(())`: If the process was successfully killed.
    /// - `Err(Error)`: If no process with the given ID exists or killing fails.s
    pub fn kill(&mut self, pid: u32) -> Result<()> {
        self.0.iter_mut().try_for_each(|(ps, _)| -> Result<()> {
            if ps.id() == pid {
                let kill = ps.kill();

                unsafe {
                    ManuallyDrop::drop(ps);
                }

                kill.map_err(|_| Error::new(ErrorKind::Internal, ""))?;
            }

            Ok(())
        })
    }

    /// Waits for all foreground processes or checks the status of background processes.
    ///
    /// This method waits for the termination of foreground processes and checks the
    /// status of background processes, collecting their exit statuses.
    ///
    /// # Returns
    /// - A vector of tuples containing:
    ///   - `u32`: The process ID.
    ///   - `std::process::ExitStatus`: The exit status of the process.
    pub fn wait(&mut self) -> Vec<(u32, std::process::ExitStatus)> {
        let mut v = Vec::with_capacity(self.0.len());

        self.0.iter_mut().for_each(|(ps, is_background)| {
            if *is_background == true {
                if let Ok(exitstatus) = ps.try_wait() {
                    if let Some(exitstatus) = exitstatus {
                        v.push((ps.id(), exitstatus));
                    }

                    unsafe {
                        ManuallyDrop::drop(ps);
                    }
                }
            } else {
                if let Ok(status) = ps.wait() {
                    v.push((ps.id(), status));
                }

                unsafe {
                    ManuallyDrop::drop(ps);
                }
            }
        });

        v
    }
}
