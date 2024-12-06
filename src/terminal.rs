use crate::{error::Error, result::Result};
use std::io::{self, Write};

/// Represents terminal cursor movement commands.
///
/// This enum provides variants for cursor control and formatting actions,
/// including moving the cursor, backspacing, and directional movements.
///
/// # Variants
/// - `Move(usize)`: Moves the cursor to a specific position (column) on the current line.
/// - `Backspace`: Simulates a backspace operation.
/// - `Left`: Moves the cursor one position to the left.
/// - `Right`: Moves the cursor one position to the right.
enum Cursor {
    Move(usize),
    Backspace,
    Left,
    Right,
}

impl Cursor {
    /// Converts the cursor movement command into an ANSI escape code.
    ///
    /// This method generates the appropriate ANSI escape sequence for the
    /// given cursor action.
    ///
    /// # Returns
    /// - A `String` containing the ANSI escape code.
    fn esc_code(&self) -> String {
        match self {
            Self::Move(position) => format!("\x1b[{position}G"),
            Self::Backspace => format!("\x08{}", " "),
            Self::Left => format!("\x1b[1D"),
            Self::Right => format!("\x1b[1C"),
        }
    }
}

/// Represents an editable line of text with cursor position tracking.
///
/// This struct maintains the state of a line of input, including the text content
/// and the current cursor position.
// ------------
// # Fields
// - `0`: Current cursor position in the line.
// - `1`: A `Vec<u8>` storing the line's content as raw bytes.
struct Line(usize, Vec<u8>);

impl Line {
    /// Creates a new, empty `Line` with the cursor at position 0.
    ///
    /// # Returns
    /// - A `Line` instance initialized with an empty buffer and cursor at the start.
    const fn new() -> Self {
        Self(0, Vec::new())
    }

    /// Inserts a character at the current cursor position.
    ///
    /// The character is added to the line's content, and the cursor moves
    /// one position to the right.
    ///
    /// # Arguments
    /// - `c`: The character to insert, represented as a `u8`.
    fn insert(&mut self, c: u8) {
        self.1.insert(self.0, c);
        self.0 += 1;
    }

    /// Deletes the character before the current cursor position.
    ///
    /// This operation moves the cursor one position to the left and removes
    /// the character at the previous position.
    fn backspace(&mut self) {
        self.1.remove(self.0 - 1);
        self.0 -= 1;
    }

    /// Moves the cursor one position to the left.
    ///
    /// The cursor does not move if it is already at the start of the line.
    fn move_left(&mut self) {
        if self.0 > 0 {
            self.0 -= 1;
        }
    }

    /// Moves the cursor one position to the right.
    ///
    /// The cursor does not move if it is already at the end of the line.
    fn move_right(&mut self) {
        if self.0 < self.1.len() {
            self.0 += 1;
        }
    }

    /// Returns the length of the line.
    ///
    /// # Returns
    /// - The number of characters in the line.
    fn len(&self) -> usize {
        self.1.len()
    }

    /// Returns the current cursor position.
    ///
    /// # Returns
    /// - The zero-based cursor position within the line.
    fn position(&self) -> usize {
        self.0
    }
}

impl ToString for Line {
    /// Converts the line content to a `String`.
    ///
    /// This method interprets the raw bytes in the line's buffer as UTF-8.
    ///
    /// # Returns
    /// - A `String` containing the line's content.
    fn to_string(&self) -> String {
        String::from_utf8_lossy(&self.1).to_string()
    }
}

/// Returns a default `libc::termios` structure for the target platform.
///
/// This function initializes a `termios` structure with default values
/// for use in terminal configuration.
///
/// # Platform-Specific Behavior
/// - On Linux, the `c_cc` array has a size of 32.
/// - On macOS, the `c_cc` array has a size of 20.
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

/// Returns a default `libc::termios` structure for the target platform.
///
/// This function initializes a `termios` structure with default values
/// for use in terminal configuration.
///
/// # Platform-Specific Behavior
/// - On Linux, the `c_cc` array has a size of 32.
/// - On macOS, the `c_cc` array has a size of 20.
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

/// Reads a single character from standard input.
///
/// This function reads one byte from the standard input using the `libc::read` system call.
/// It blocks until input is available.
///
/// # Returns
/// - `Some(u8)`: The byte read from the input.
/// - `None`: If an error occurs or no input is available.
///
/// # Safety
/// - Uses `unsafe` to call the `libc::read` function. The caller must ensure the safety
///   of this operation in their context.
unsafe fn get_char() -> Option<u8> {
    let mut code = vec![0; 1];

    if unsafe { libc::read(0, code.as_mut_ptr() as *mut libc::c_void, 1) } <= 0 {
        None?;
    }

    Some(code[0])
}

pub struct Terminal {
    termios: libc::termios,
    prompt: String,
}

impl Terminal {
    pub fn new() -> Self {
        Self {
            termios: termios(),
            prompt: String::from("# "),
        }
    }

    /// Sets the terminal to raw mode.
    ///
    /// In raw mode, the terminal input is unprocessed, meaning that all
    /// input is passed directly to the program without being modified
    /// or interpreted by the terminal driver. This is typically used
    /// for interactive terminal applications or real-time input processing.
    ///
    /// # Behavior
    /// - Disables canonical mode (`ICANON`), so input is available immediately
    ///   without waiting for a newline.
    /// - Disables input echo (`ECHO`), so characters entered by the user are not displayed on the screen.
    /// - Disables extended input processing (`IEXTEN`), disabling additional terminal features.
    /// - Disables signal generation (`ISIG`), so signals like `Ctrl+C` are ignored.
    /// - Configures the input to require at least one character (`VMIN = 1`) and no timeout (`VTIME = 0`).
    ///
    /// # Safety
    /// - This function uses `unsafe` blocks because it interacts directly
    ///   with the terminal via libc system calls (`tcgetattr`, `tcsetattr`, `fcntl`).
    /// - The caller must ensure that the terminal state is restored to its
    ///   original configuration when the program exits to avoid leaving the
    ///   terminal in an inconsistent state.
    ///
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

    /// Resets the terminal mode to the original state.
    ///
    /// This function restores the terminal settings to the state saved in `self.termios`.
    /// It is typically used to revert the changes made by `set_raw_mode` after raw mode
    /// operations are completed.
    ///
    /// # Safety
    /// - This function uses `unsafe` to call `libc::tcsetattr`. The caller must ensure
    ///   that the `termios` field contains a valid and previously saved terminal state.
    ///
    unsafe fn reset_raw_mode(&mut self) {
        unsafe {
            libc::tcsetattr(0, 0, &self.termios);
        }
    }

    /// Reads a line of input from the terminal, supporting raw mode editing.
    ///
    /// This function reads user input interactively in raw mode, allowing for real-time
    /// processing of special keys (e.g., arrow keys, backspace, etc.). It supports line editing
    /// features such as:
    /// - Inserting and deleting characters.
    /// - Moving the cursor with arrow keys.
    /// - Exiting on `Ctrl+C`.
    /// - Submitting input on `Enter`.
    ///
    /// # Behavior
    /// - Temporarily sets the terminal to raw mode using `set_raw_mode`.
    /// - Displays the prompt stored in `self.prompt` before reading input.
    /// - Processes user input character by character, handling special key sequences.
    /// - Returns the final input line as a `String` upon pressing `Enter`.
    ///
    /// # Key Bindings
    /// - `Ctrl+C`: Exits the program immediately.
    /// - `Enter`: Completes the line and returns the input.
    /// - `Backspace`: Deletes the character before the cursor.
    /// - `Arrow Keys`:
    ///   - Left/Right: Moves the cursor within the line.
    ///   - Up/Down: (Currently not implemented.)
    /// - Other characters: Inserted into the current cursor position.
    ///
    /// # Safety
    /// - This function uses `unsafe` blocks to call `libc` functions directly (`tcsetattr`, `exit`).
    /// - Ensure proper handling of the terminal state to prevent leaving the terminal in raw mode.
    ///
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
