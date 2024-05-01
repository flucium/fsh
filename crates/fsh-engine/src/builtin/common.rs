use std::process;

/// Exits the process with the given exit code.
pub fn exit(code: i32) {
    process::exit(code)
}

/// Aborts the process.
pub fn abort() {
    process::abort()
}
