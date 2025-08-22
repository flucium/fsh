use crate::{error::*, result::*};
use std::io::{self, Write};

/// Cursor movement operations used for terminal editing.
enum Cursor {
    /// Moves the cursor to the specified absolute column position.
    Move(usize),

    /// Deletes the character to the left of the cursor.
    Backspace,

    /// Moves the cursor one position to the left.
    Left,

    /// Moves the cursor one position to the right.
    Right,
}

impl Cursor {
    /// Returns the ANSI escape code corresponding to this cursor operation.
    fn esc_code(&self) -> String {
        match self {
            Self::Move(position) => format!("\x1b[{position}G"),
            Self::Backspace => format!("\x08 "),
            Self::Left => format!("\x1b[1D"),
            Self::Right => format!("\x1b[1C"),
        }
    }
}

/// A line buffer storing characters and the cursor position.
struct Line(usize, Vec<u8>);

impl Line {
    /// Constructs a new, empty `Line`.
    const fn new() -> Self {
        Self(0, Vec::new())
    }

    /// Inserts a character at the cursor position.
    fn insert(&mut self, c: u8) {
        self.1.insert(self.0, c);
        self.0 += 1;
    }

    /// Removes the character to the left of the cursor.
    fn backspace(&mut self) {
        self.1.remove(self.0 - 1);
        self.0 -= 1;
    }

    /// Moves the cursor one position to the left.
    fn move_left(&mut self) {
        if self.0 > 0 {
            self.0 -= 1;
        }
    }

    /// Moves the cursor one position to the right.
    fn move_right(&mut self) {
        if self.0 < self.1.len() {
            self.0 += 1;
        }
    }

    /// Returns the length.
    fn len(&self) -> usize {
        self.1.len()
    }

    /// Returns the current cursor position.
    fn position(&self) -> usize {
        self.0
    }
}

impl ToString for Line {
    fn to_string(&self) -> String {
        String::from_utf8_lossy(&self.1).to_string()
    }
}

/// Returns an empty `termios` struct for Linux.
#[cfg(target_os = "linux")]
fn termios() -> libc::termios {
    libc::termios {
        c_line: 0,
        c_cc: [0; 32],
        c_ispeed: 0,
        c_ospeed: 0,
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
    }
}

/// Returns an empty `termios` struct for macOS.
#[cfg(target_os = "macos")]
fn termios() -> libc::termios {
    libc::termios {
        c_cc: [0u8; 20],
        c_ispeed: 0,
        c_ospeed: 0,
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
    }
}

/// Reads a single byte from stdin.
///
/// # Safety
/// Calls the `libc::read` system call directly and is unsafe by nature.
///
/// # Returns
/// - `Some(u8)` if a character was read.  
/// - `None` if the read failed or reached EOF.
unsafe fn get_char() -> Option<u8> {
    let mut code = vec![0; 1];

    if unsafe { libc::read(0, code.as_mut_ptr() as *mut libc::c_void, 1) } <= 0 {
        None?;
    }

    Some(code[0])
}

/// A terminal with raw mode input and line editing.
pub struct Terminal {
    termios: libc::termios,

    prompt: String,
}

impl Terminal {
    /// Creates a new terminal with default settings.
    pub fn new() -> Self {
        Self {
            termios: termios(),
            prompt: String::default(),
        }
    }

    /// Sets the prompt string displayed before input.
    pub fn set_prompt(&mut self, prompt: impl Into<String>) {
        self.prompt = prompt.into();
    }

    /// Enables raw mode for the terminal.
    ///
    /// # Safety
    /// Modifies terminal attributes using `libc::tcsetattr`.
    unsafe fn set_raw_mode(&mut self) {
        unsafe { libc::tcgetattr(0, &mut self.termios) };

        let mut raw = self.termios;

        raw.c_lflag = raw.c_lflag & !(libc::ICANON | libc::ECHO | libc::IEXTEN | libc::ISIG);

        raw.c_cc[libc::VTIME] = 0;

        raw.c_cc[libc::VMIN] = 1;

        unsafe {
            libc::tcsetattr(0, 0, &raw);
            libc::fcntl(0, libc::F_SETFL);
        }
    }

    /// Restores the terminal's original attributes.
    ///
    /// # Safety
    /// Modifies terminal attributes using `libc::tcsetattr`.
    unsafe fn reset_raw_mode(&mut self) {
        unsafe {
            libc::tcsetattr(0, 0, &self.termios);
        }
    }

    /// Reads a line of input from the terminal with basic line editing.
    ///
    /// Supports:
    /// - Character insertion
    /// - Backspace
    /// - Left/right cursor movement
    /// - Ctrl-C to exit
    /// - Enter to submit
    ///
    /// # Returns
    /// - `Ok(String)` with the line entered by the user.  
    /// - `Err(Error)` if input or output fails.
    pub fn read_line(&mut self) -> Result<String> {
        unsafe { self.set_raw_mode() };

        let mut stdout = io::stdout().lock();

        stdout
            .write_all(self.prompt.as_bytes())
            .map_err(|_| Error::new(ErrorKind::Interrupted, "failed to write prompt"))?;

        let mut line = Line::new();

        loop {
            stdout
                .flush()
                // .map_err(|_| Error::new(ErrorKind::Internal, "failed to flush stdout"))?;
                .map_err(|_| Error::INTERNAL)?;

            let ch = match unsafe { get_char() } {
                Some(ch) => ch,
                None => continue,
            };

            match ch {
                3 => {
                    stdout.write_all("\n".as_bytes()).map_err(|_| {
                        Error::new(ErrorKind::Interrupted, "failed to write newline")
                    })?;

                    unsafe { self.reset_raw_mode() };

                    unsafe {
                        libc::exit(0);
                    }
                }

                10 => {
                    break;
                }

                27 => {
                    if unsafe { get_char() }.unwrap_or(0) != 91 {
                        continue;
                    }

                    match unsafe { get_char() }.unwrap_or(0) {
                        65 => {}

                        66 => {}

                        67 => {
                            if line.position() < line.len() {
                                line.move_right();

                                stdout
                                    .write_all(format!("{}", Cursor::Right.esc_code()).as_bytes())
                                    .map_err(|_| {
                                        Error::new(
                                            ErrorKind::Interrupted,
                                            "failed to move cursor right",
                                        )
                                    })?;
                            }
                        }

                        68 => {
                            if line.position() > 0 {
                                stdout
                                    .write_all(format!("{}", Cursor::Left.esc_code()).as_bytes())
                                    .map_err(|_| {
                                        Error::new(
                                            ErrorKind::Interrupted,
                                            "failed to move cursor left",
                                        )
                                    })?;

                                line.move_left();
                            }
                        }

                        _ => {
                            continue;
                        }
                    }
                }

                127 => {
                    if line.position() <= 0 {
                        continue;
                    }

                    stdout
                        .write_all(format!("{}", Cursor::Left.esc_code()).as_bytes())
                        .map_err(|_| {
                            Error::new(ErrorKind::Interrupted, "failed to move cursor left")
                        })?;

                    stdout.write_all(b" ").map_err(|_| {
                        Error::new(ErrorKind::Interrupted, "failed to erase character")
                    })?;

                    stdout
                        .write_all(format!("{}", Cursor::Backspace.esc_code()).as_bytes())
                        .map_err(|_| Error::new(ErrorKind::Interrupted, "failed to backspace"))?;

                    stdout
                        .write_all(format!("{}", Cursor::Left.esc_code()).as_bytes())
                        .map_err(|_| {
                            Error::new(ErrorKind::Interrupted, "failed to move cursor left")
                        })?;

                    line.backspace();

                    stdout
                        .write_all(format!("\r{}{}", self.prompt, line.to_string()).as_bytes())
                        .map_err(|_| {
                            Error::new(
                                ErrorKind::Interrupted,
                                "failed to redraw line after backspace",
                            )
                        })?;

                    stdout
                        .write_all(
                            format!(
                                "{}",
                                Cursor::Move(self.prompt.len() + line.position() + 1).esc_code()
                            )
                            .as_bytes(),
                        )
                        .map_err(|_| {
                            Error::new(
                                ErrorKind::Interrupted,
                                "failed to reposition cursor after backspace",
                            )
                        })?;
                }

                _ => {
                    line.insert(ch);

                    for i in 0..line.len() {
                        if i != 0 {
                            stdout
                                .write_all(format!("{}", Cursor::Backspace.esc_code()).as_bytes())
                                .map_err(|_| {
                                    Error::new(
                                        ErrorKind::Interrupted,
                                        "failed to backspace during overwrite",
                                    )
                                })?;
                        }
                    }

                    stdout
                        .write_all(format!("\r{}{}", self.prompt, line.to_string()).as_bytes())
                        .map_err(|_| {
                            Error::new(ErrorKind::Interrupted, "failed to redraw line after insert")
                        })?;

                    if line.position() < line.len() {
                        let move_position = self.prompt.len() + line.position() + 1;

                        stdout
                            .write_all(
                                format!("{}", Cursor::Move(move_position).esc_code()).as_bytes(),
                            )
                            .map_err(|_| {
                                Error::new(
                                    ErrorKind::Interrupted,
                                    "failed to reposition cursor after insert",
                                )
                            })?;
                    }
                }
            }
        }

        unsafe {
            self.reset_raw_mode();
        }

        stdout
            .write_all(b"\n")
            .map_err(|_| Error::new(ErrorKind::Interrupted, "failed to write final newline"))?;

        stdout
            .flush()
            .map_err(|_| Error::new(ErrorKind::Other, "failed to flush stdout"))?;

        let line = line.to_string();

        Ok(line)
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        unsafe {
            self.reset_raw_mode();
        }
    }
}
