use crate::{error::Error, result::Result};
use std::{mem::ManuallyDrop, process};

pub struct ProcessHandler(Vec<(ManuallyDrop<process::Child>, bool)>);

impl ProcessHandler {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn new_with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    pub fn push(&mut self, ps: process::Child, is_background: bool) -> u32 {
        let pid = ps.id();

        self.0.push((ManuallyDrop::new(ps), is_background));

        pid
    }

    pub fn pop(&mut self) -> Option<std::process::Child> {
        self.0.pop().map(|(ps, _)| ManuallyDrop::into_inner(ps))
    }

    pub fn get(&self, pid: u32) -> Option<&std::process::Child> {
        self.0
            .iter()
            .find(|(ps, _)| ps.id() == pid)
            .map(|(ps, _)| &**ps)
    }

    pub fn get_mut(&mut self, pid: u32) -> Option<&mut std::process::Child> {
        self.0
            .iter_mut()
            .find(|(ps, _)| ps.id() == pid)
            .map(|(ps, _)| &mut **ps)
    }

    pub fn remove(&mut self, pid: u32) -> Result<()> {
        let index = self
            .0
            .iter()
            .position(|(ps, _)| ps.id() == pid)
            .ok_or_else(|| Error::NOT_IMPLEMENTED)?;

        unsafe {
            ManuallyDrop::drop(&mut self.0.remove(index).0);
        }

        Ok(())
    }

    pub fn entries(&self) -> Vec<&std::process::Child> {
        self.0.iter().map(|(ps, _)| &**ps).collect()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    pub fn kill(&mut self, pid: u32) -> Result<()> {
        self.0.iter_mut().try_for_each(|(ps, _)| -> Result<()> {
            if ps.id() == pid {
                let kill = ps.kill();

                unsafe {
                    ManuallyDrop::drop(ps);
                }

                kill.map_err(|_| Error::NOT_IMPLEMENTED)?;
            }

            Ok(())
        })
    }

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
