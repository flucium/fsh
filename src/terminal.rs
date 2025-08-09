use crate::{error::Error, result::Result};
use std::io::{self, Write};

/// Represents a terminal cursor control action.
///
/// Each variant corresponds to a specific movement or editing
/// operation, typically translated into ANSI escape sequences.
enum Cursor {
    /// Moves the cursor to an absolute horizontal position (1-based index).
    Move(usize),

    /// Simulates a backspace by moving left and overwriting with a space.
    Backspace,

    /// Moves the cursor one position to the left.
    Left,

    /// Moves the cursor one position to the right.
    Right,
}

impl Cursor {
    /// Returns the ANSI escape sequence corresponding to the cursor action.
    ///
    /// # Returns
    /// A `String` containing the escape code for the given cursor movement or action.
    fn esc_code(&self) -> String {
        match self {
            Self::Move(position) => format!("\x1b[{position}G"),
            Self::Backspace => format!("\x08 "),
            Self::Left => format!("\x1b[1D"),
            Self::Right => format!("\x1b[1C"),
        }
    }
}

/// Represents an editable line buffer with a cursor position.
///
/// The first field (`usize`) stores the current cursor position,
/// and the second field (`Vec<u8>`) stores the line content as raw bytes.
/// This structure is typically used for interactive line editing in the terminal.
struct Line(usize, Vec<u8>);

impl Line {
    /// Creates a new `Line`.
    const fn new() -> Self {
        Self(0, Vec::new())
    }

    /// Inserts a byte at the current cursor position and moves the cursor forward by one.
    ///
    /// # Arguments
    /// - `c`: The byte to insert. byte: char byte (u8).
    fn insert(&mut self, c: u8) {
        self.1.insert(self.0, c);
        self.0 += 1;
    }

    /// Deletes the byte immediately before the cursor and moves the cursor backward by one.
    fn backspace(&mut self) {
        self.1.remove(self.0 - 1);
        self.0 -= 1;
    }

    /// Moves the cursor one position to the left, if possible.
    fn move_left(&mut self) {
        if self.0 > 0 {
            self.0 -= 1;
        }
    }

    /// Moves the cursor one position to the right, if possible.
    fn move_right(&mut self) {
        if self.0 < self.1.len() {
            self.0 += 1;
        }
    }

    /// Returns the number of bytes in the line buffer.
    fn len(&self) -> usize {
        self.1.len()
    }

    /// Returns the current cursor position within the line buffer.
    fn position(&self) -> usize {
        self.0
    }
}

impl ToString for Line {
    fn to_string(&self) -> String {
        String::from_utf8_lossy(&self.1).to_string()
    }
}

/// Returns an empty, zero-initialized `libc::termios` structure for Linux.
///
/// This function provides a default-initialized `termios` instance,
/// which can be modified before applying new terminal settings.
///
/// This version is compiled only on Linux (`target_os = "linux"`).
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

/// Returns an empty, zero-initialized `libc::termios` structure for macOS.
///
/// This function provides a default-initialized `termios` instance,
/// which can be modified before applying new terminal settings.
///
/// This version is compiled only on macOS (`target_os = "macos"`).
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

/// Reads a single byte from standard input (`stdin`) in raw mode.
///
/// This function attempts to read exactly one byte from file descriptor `0`
/// (standard input) using the `libc::read` system call.
///
/// # Safety
/// This function is `unsafe` because it performs a raw FFI call to `libc::read`
/// and dereferences a raw pointer. The caller must ensure that the terminal
/// is in a mode that allows single-byte reads (e.g., raw or non-canonical mode),
/// and that it is safe to perform this operation in the current context.
///
/// # Returns
/// - `Some(u8)` containing the read byte if successful.
/// - `None` if no data was read or an error occurred.
///
/// # Platform Behavior
/// - On blocking terminals, this call will wait until at least one byte is available.
/// - On non-blocking terminals, this call may return immediately with `None`.
unsafe fn get_char() -> Option<u8> {
    let mut code = vec![0; 1];

    if unsafe { libc::read(0, code.as_mut_ptr() as *mut libc::c_void, 1) } <= 0 {
        None?;
    }

    Some(code[0])
}

/// Represents an interactive terminal interface with raw input capabilities.
///
/// This struct manages terminal settings, prompt display, and
/// interactive line editing (including cursor movement and backspace handling).
pub struct Terminal {
    /// Stores the terminal's original `termios` settings for restoration.
    termios: libc::termios,

    /// The prompt string displayed before user input.
    prompt: String,
}

impl Terminal {
    /// Creates a new `Terminal`.
    ///
    /// The `termios` field is initialized using the platform-specific `termios()` helper.
    /// The prompt is initially set to an empty string.
    pub fn new() -> Self {
        Self {
            termios: termios(),
            prompt: String::default(),
        }
    }

    /// Sets the prompt string that will be displayed before user input.
    ///
    /// # Arguments
    /// - `prompt`: The prompt text to display.
    pub fn set_prompt(&mut self, prompt: impl Into<String>) {
        self.prompt = prompt.into();
    }

    /// Enables raw mode on the terminal, disabling canonical input and echo.
    ///
    /// In raw mode:
    /// - Input is read byte-by-byte without waiting for a newline.
    /// - Input is not echoed back to the terminal automatically.
    /// - Signal-generating keys (e.g., Ctrl+C) are disabled.
    ///
    /// # Safety
    /// This method calls `libc::tcgetattr` and `libc::tcsetattr` directly and
    /// modifies terminal I/O settings. It must be paired with `reset_raw_mode`
    /// to restore the original state.
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

    /// Restores the terminal to its original `termios` settings.
    ///
    /// # Safety
    /// Directly calls `libc::tcsetattr` to reset terminal I/O settings.
    unsafe fn reset_raw_mode(&mut self) {
        unsafe {
            libc::tcsetattr(0, 0, &self.termios);
        }
    }

    /// Reads a line of input from the terminal with interactive editing support.
    ///
    /// This function:
    /// - Enables raw mode for single-character input.
    /// - Displays the prompt.
    /// - Supports cursor movement (left/right arrow keys).
    /// - Handles backspace.
    /// - Exits immediately on Ctrl+C (`SIGINT` equivalent).
    ///
    /// Input is returned as soon as the Enter key (`\n`) is pressed.
    /// The terminal is always restored to its original state before returning.
    ///
    /// # Returns
    /// - `Ok(String)` containing the user-entered line.
    /// - `Err(Error::NOT_IMPLEMENTED)` if any I/O operation fails.
    pub fn read_line(&mut self) -> Result<String> {
        unsafe { self.set_raw_mode() };

        let mut stdout = io::stdout().lock();

        stdout
            .write_all(self.prompt.as_bytes())
            .map_err(|_| Error::NOT_IMPLEMENTED)?;

        let mut line = Line::new();

        loop {
            stdout.flush().map_err(|_| Error::NOT_IMPLEMENTED)?;

            let ch = match unsafe { get_char() } {
                Some(ch) => ch,
                None => continue,
            };

            match ch {
                3 => {
                    stdout
                        .write_all("\n".as_bytes())
                        .map_err(|_| Error::NOT_IMPLEMENTED)?;

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
                                    .map_err(|_| Error::NOT_IMPLEMENTED)?;
                            }
                        }

                        68 => {
                            if line.position() > 0 {
                                stdout
                                    .write_all(format!("{}", Cursor::Left.esc_code()).as_bytes())
                                    .map_err(|_| Error::NOT_IMPLEMENTED)?;

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
                        .map_err(|_| Error::NOT_IMPLEMENTED)?;

                    stdout.write_all(b" ").map_err(|_| Error::NOT_IMPLEMENTED)?;

                    stdout
                        .write_all(format!("{}", Cursor::Backspace.esc_code()).as_bytes())
                        .map_err(|_| Error::NOT_IMPLEMENTED)?;

                    stdout
                        .write_all(format!("{}", Cursor::Left.esc_code()).as_bytes())
                        .map_err(|_| Error::NOT_IMPLEMENTED)?;

                    line.backspace();

                    stdout
                        .write_all(format!("\r{}{}", self.prompt, line.to_string()).as_bytes())
                        .map_err(|_| Error::NOT_IMPLEMENTED)?;

                    stdout
                        .write_all(
                            format!(
                                "{}",
                                Cursor::Move(self.prompt.len() + line.position() + 1).esc_code()
                            )
                            .as_bytes(),
                        )
                        .map_err(|_| Error::NOT_IMPLEMENTED)?;
                }

                _ => {
                    line.insert(ch);

                    for i in 0..line.len() {
                        if i != 0 {
                            stdout
                                .write_all(format!("{}", Cursor::Backspace.esc_code()).as_bytes())
                                .map_err(|_| Error::NOT_IMPLEMENTED)?;
                        }
                    }

                    stdout
                        .write_all(format!("\r{}{}", self.prompt, line.to_string()).as_bytes())
                        .map_err(|_| Error::NOT_IMPLEMENTED)?;

                    if line.position() < line.len() {
                        let move_position = self.prompt.len() + line.position() + 1;
                        stdout
                            .write_all(
                                format!("{}", Cursor::Move(move_position).esc_code()).as_bytes(),
                            )
                            .map_err(|_| Error::NOT_IMPLEMENTED)?;
                    }
                }
            }
        }

        unsafe {
            self.reset_raw_mode();
        }

        stdout
            .write_all(b"\n")
            .map_err(|_| Error::NOT_IMPLEMENTED)?;

        stdout.flush().map_err(|_| Error::NOT_IMPLEMENTED)?;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_position() {
        let mut line = Line::new();

        for c in b"Hello" {
            line.insert(*c as u8);
        }
        assert_eq!(line.position(), 5);
        assert_eq!(line.len(), 5);
    }

    #[test]
    fn test_line_len() {
        let mut line = Line::new();

        for c in b"Hello" {
            line.insert(*c as u8);
        }
        assert_eq!(line.len(), 5);
    }

    #[test]
    fn test_line_backspace() {
        let mut line = Line::new();

        for c in b"Hello" {
            line.insert(*c as u8);
        }
        assert_eq!(line.position(), 5);
        assert_eq!(line.len(), 5);

        for _ in 0..5 {
            line.backspace();
        }
        assert_eq!(line.position(), 0);
        assert_eq!(line.len(), 0);
    }

    #[test]
    fn test_line_move_left() {
        let mut line = Line::new();

        for c in b"Hello" {
            line.insert(*c as u8);
        }
        assert_eq!(line.position(), 5);

        line.move_left();
        assert_eq!(line.position(), 4);

        for _ in 0..100 {
            line.move_left();
        }
        assert_eq!(line.position(), 0);
    }

    #[test]
    fn test_line_move_right() {
        let mut line = Line::new();

        for c in b"Hello" {
            line.insert(*c as u8);
            line.move_left();
        }
        assert_eq!(line.position(), 0);

        line.move_right();
        assert_eq!(line.position(), 1);

        for _ in 0..100 {
            line.move_right();
        }
        assert_eq!(line.position(), 5);
    }

    #[test]
    fn test_line_insert() {
        let mut line = Line::new();

        for c in b"Hello" {
            line.insert(*c as u8);
        }

        assert_eq!(line.len(), 5);
        assert_eq!(line.1, [72, 101, 108, 108, 111]);
    }

    #[test]
    fn test_line_to_string() {
        let mut line = Line::new();

        for c in b"Hello" {
            line.insert(*c as u8);
        }

        assert_eq!(line.to_string(), "Hello");
    }

    #[test]
    fn test_cursor_move() {
        let cursor = Cursor::Move(10);
        assert_eq!(cursor.esc_code(), "\x1b[10G");
    }

    #[test]
    fn test_cursor_backspace() {
        let cursor = Cursor::Backspace;
        assert_eq!(cursor.esc_code(), "\x08 ");
    }

    #[test]
    fn test_cursor_left() {
        let cursor = Cursor::Left;
        assert_eq!(cursor.esc_code(), "\x1b[1D");
    }

    #[test]
    fn test_cursor_right() {
        let cursor = Cursor::Right;
        assert_eq!(cursor.esc_code(), "\x1b[1C");
    }
}
