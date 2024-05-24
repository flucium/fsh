
macro_rules! fsh_eprintln {
    () => {};
    ($($arg:expr)*) => {
        eprintln!("fsh: {}", format!($($arg)*));
    };
    ($($arg:expr)*, $kind:expr) => {
        eprintln!("fsh: {} ({})", format!($($arg)*), $kind.as_str());
    };
}

macro_rules! fsh_panic {
    () => {};
    ($($arg:expr)*) => {
        panic!("fsh: {}", format!($($arg)*));
    };
    ($($arg:expr)*, $kind:expr) => {
        panic!("fsh: {} ({})", format!($($arg)*), $kind.as_str());
    };
}