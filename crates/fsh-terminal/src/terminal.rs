use std::io;
use std::io::Write;

use super::ascii;
use super::line::*;
use fsh_common::{Error, ErrorKind, Result};

/// Get character (u8) from stdin.
#[inline]
fn get_char() -> Option<u8> {
    let code = [0; 1];

    let n = unsafe { libc::read(0, code.as_ptr() as *mut libc::c_void, 1) };

    if n <= 0 {
        return None;
    }

    Some(code[0])
}

/// Termios struct (Linux).
#[inline]
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

/// Termios struct (macOS).
#[inline]
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

/// Terminal obj.
pub struct Terminal {
    termios: libc::termios,
    prompt: String,
}

impl Terminal {
    /// Create a new Terminal obj.
    pub fn new() -> Self {
        Self {
            termios: termios(),
            prompt: String::default(),
        }
    }

    /// Update terminal prompt.
    pub fn update_prompt(&mut self, prompt: impl Into<String>) {
        self.prompt = prompt.into();
    }

    /// Set terminal to raw mode.
    fn set_raw_mode(&mut self) {
        unsafe { libc::tcgetattr(0, &mut self.termios) };

        let mut raw = self.termios;

        raw.c_lflag = raw.c_lflag & !(libc::ICANON | libc::ECHO | libc::IEXTEN | libc::ISIG);
        // raw.c_lflag = raw.c_lflag & !(libc::ICANON | libc::ECHO );

        raw.c_cc[libc::VTIME] = 0;

        raw.c_cc[libc::VMIN] = 1;

        unsafe {
            libc::tcsetattr(0, 0, &raw);
            libc::fcntl(0, libc::F_SETFL);
        }
    }

    /// Reset terminal to raw mode.
    fn reset_raw_mode(&mut self) {
        unsafe {
            libc::tcsetattr(0, 0, &self.termios);
        }
    }

    /// Read a line from stdin.
    pub fn read_line(&mut self) -> Result<String> {
        // Set terminal to raw mode.
        self.set_raw_mode();

        let mut line = Line::new();

        let mut stdout = io::stdout().lock();

        stdout
            .write_all(self.prompt.as_bytes())
            .map_err(|_| Error::new(ErrorKind::Unknown, "unknown error"))?;

        loop {
            // Stdout flush.
            stdout
                .flush()
                .map_err(|_| Error::new(ErrorKind::Unknown, "unknown error"))?;

            // Get character.
            let ch = if let Some(ch) = get_char() {
                ch
            } else {
                continue;
            };

            match ch {
                // Exit.
                3 => {
                    // Write newline to
                    stdout
                        .write_all("\n".as_bytes())
                        .map_err(|_| Error::new(ErrorKind::Unknown, "unknown error"))?;

                    // Reset terminal to raw mode.
                    self.reset_raw_mode();

                    // Exit 0.
                    unsafe {
                        libc::exit(0);
                    }
                }

                // Enter.
                10 => {
                    break;
                }

                // Special characters.
                27 => {
                    if get_char().unwrap_or(0) != 91 {
                        continue;
                    }

                    // Arrow keys.
                    match get_char().unwrap_or(0) {
                        // Up.
                        65 => {}

                        // Down.
                        66 => {}

                        // Right.
                        67 => {
                            if line.position() < line.len() {
                                line.move_right();

                                stdout
                                    .write_all(
                                        format!("{}", ascii::Cursor::Right.get_esc_code())
                                            .as_bytes(),
                                    )
                                    .map_err(|_| Error::new(ErrorKind::Unknown, "unknown error"))?;
                            }
                        }

                        // Left.
                        68 => {
                            if line.position() > 0 {
                                stdout
                                    .write_all(
                                        format!("{}", ascii::Cursor::Left.get_esc_code())
                                            .as_bytes(),
                                    )
                                    .map_err(|_| Error::new(ErrorKind::Unknown, "unknown error"))?;

                                line.move_left();
                            }
                        }

                        // Unknown.
                        _ => {
                            continue;
                        }
                    }
                }

                // Backspace.
                127 => {
                    if line.position() <= 0 {
                        continue;
                    }

                    for i in 0..line.len() {
                        if i != 0 {
                            stdout
                                .write_all(
                                    format!("{}", ascii::Cursor::Backspace.get_esc_code())
                                        .as_bytes(),
                                )
                                .map_err(|_| Error::new(ErrorKind::Unknown, "unknown error"))?;
                        }
                    }

                    stdout
                        .write_all(format!("\r{}{}", self.prompt, line.to_string()).as_bytes())
                        .map_err(|_| Error::new(ErrorKind::Unknown, "unknown error"))?;

                    line.backspace();

                    stdout
                        .write_all(
                            format!("{}", ascii::Cursor::Backspace.get_esc_code()).as_bytes(),
                        )
                        .map_err(|_| Error::new(ErrorKind::Unknown, "unknown error"))?;

                    stdout
                        .write_all(format!("\r{}{}", self.prompt, line.to_string(),).as_bytes())
                        .map_err(|_| Error::new(ErrorKind::Unknown, "unknown error"))?;

                    if line.position() < line.len() {
                        let move_position = self.prompt.len() + line.position() - 1;

                        stdout
                            .write_all(
                                format!("{}", ascii::Cursor::Move(move_position).get_esc_code())
                                    .as_bytes(),
                            )
                            .map_err(|_| Error::new(ErrorKind::Unknown, "unknown error"))?;
                    }
                }

                // Insert character.
                _ => {
                    line.insert(ch);

                    for i in 0..line.len() {
                        if i != 0 {
                            stdout
                                .write_all(
                                    format!("{}", ascii::Cursor::Backspace.get_esc_code())
                                        .as_bytes(),
                                )
                                .map_err(|_| Error::new(ErrorKind::Unknown, "unknown error"))?;
                        }
                    }
                }
            }
        }

        // Reset terminal to raw mode.
        self.reset_raw_mode();

        // Write newline to stdout.
        stdout
            .write_all(b"\n")
            .map_err(|_| Error::new(ErrorKind::Unknown, "unknown error"))?;

        // Stdout flush.
        stdout
            .flush()
            .map_err(|_| Error::new(ErrorKind::Unknown, "unknown error"))?;

        // Convert line to string.
        let line = line.to_string();
        
        Ok(line)
    }
}