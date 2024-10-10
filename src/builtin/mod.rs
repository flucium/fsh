use std::{env, ffi::OsStr, process};

use crate::{error::Error, result::Result, state::State};

pub fn exit(code: i32) {
    process::exit(code)
}

pub fn abort() {
    process::abort()
}

pub fn cd<S: AsRef<OsStr> + ?Sized>(p: &S, state: &mut State) -> Result<()> {
    let current_path = state
        .current_dir()
        .canonicalize()
        .map_err(|_| Error::NOT_IMPLEMENTED)?;

    let path = analyze(&current_path, p)?;

    state.current_dir_mut().clear();

    state.current_dir_mut().push(path.clone());

    env::set_current_dir(path).map_err(|_| Error::NOT_IMPLEMENTED)?;

    Ok(())
}

fn analyze<A: AsRef<OsStr> + ?Sized, B: AsRef<OsStr> + ?Sized>(
    current: &A,
    target: &B,
) -> Result<std::path::PathBuf> {
    let current = std::path::Path::new(current);

    let target = std::path::Path::new(target);

    if current.is_dir() == false {
        Err(Error::NOT_IMPLEMENTED)?
    }

    let path = std::path::Path::new(current)
        .join(std::path::Path::new(target))
        .canonicalize()
        .map_err(|_| Error::NOT_IMPLEMENTED)?;

    if path.is_file() {
        Err(Error::NOT_IMPLEMENTED)?
    }

    Ok(path)
}
