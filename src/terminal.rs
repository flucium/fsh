use crate::{error::Error, result::Result};
use std::io::{self, Write};

enum Cursor {
    Move(usize),

    Backspace,

    Left,

    Right,
}

impl Cursor {
    fn esc_code(&self) -> String {
        match self {
            Self::Move(position) => format!("\x1b[{position}G"),
            Self::Backspace => format!("\x08{}", " "),
            Self::Left => format!("\x1b[1D"),
            Self::Right => format!("\x1b[1C"),
        }
    }
}

struct Line(usize, Vec<u8>);

impl Line {
    const fn new() -> Self {
        Self(0, Vec::new())
    }

    fn insert(&mut self, c: u8) {
        self.1.insert(self.0, c);
        self.0 += 1;
    }

    fn backspace(&mut self) {
        self.1.remove(self.0 - 1);
        self.0 -= 1;
    }

    fn move_left(&mut self) {
        if self.0 > 0 {
            self.0 -= 1;
        }
    }

    fn move_right(&mut self) {
        if self.0 < self.1.len() {
            self.0 += 1;
        }
    }

    fn len(&self) -> usize {
        self.1.len()
    }

    fn position(&self) -> usize {
        self.0
    }
}

impl ToString for Line {
    fn to_string(&self) -> String {
        String::from_utf8_lossy(&self.1).to_string()
    }
}

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

fn get_char() -> Option<u8> {
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

    fn set_raw_mode(&mut self) {
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

    fn reset_raw_mode(&mut self) {
        unsafe {
            libc::tcsetattr(0, 0, &self.termios);
        }
    }

    pub fn read_line(&mut self) -> Result<String> {
        self.set_raw_mode();

        let mut stdout = io::stdout().lock();

        stdout
            .write_all(self.prompt.as_bytes())
            .map_err(|_| Error::NOT_IMPLEMENTED)?;

        let mut line = Line::new();

        loop {
            stdout.flush().map_err(|_| Error::NOT_IMPLEMENTED)?;

            let ch = match get_char() {
                Some(ch) => ch,
                None => continue,
            };

            match ch {
                3 => {
                    stdout
                        .write_all("\n".as_bytes())
                        .map_err(|_| Error::NOT_IMPLEMENTED)?;

                    self.reset_raw_mode();

                    unsafe {
                        libc::exit(0);
                    }
                }

                10 => {
                    break;
                }

                27 => {
                    if get_char().unwrap_or(0) != 91 {
                        continue;
                    }

                    match get_char().unwrap_or(0) {
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

        self.reset_raw_mode();

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
        self.reset_raw_mode();
    }
}
