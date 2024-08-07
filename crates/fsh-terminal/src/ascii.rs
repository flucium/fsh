/// Represents the ASCII escape codes for the cursor.
pub(super) enum Cursor {
    /// Move the cursor to the specified position.
    Move(usize),

    /// Move the cursor back one position.
    Backspace,

    /// Move the cursor left one position.
    Left,

    /// Move the cursor right one position.
    Right,
    // ClearLine,
}

impl Cursor {
    /// Returns the escape code for the cursor.
    pub(super) fn get_esc_code(&self) -> String {
        return match &self {
            Cursor::Move(position) => format!("\x1b[{position}G"),
            Cursor::Backspace => format!("\x08{}", " "),
            Cursor::Left => format!("\x1b[1D"),
            Cursor::Right => format!("\x1b[1C"),
            // Cursor::ClearLine => format!("\x1b[2K"),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cursor_get_esc_code() {
        let cursor = Cursor::Move(5);
        assert_eq!(cursor.get_esc_code(), "\x1b[5G");

        let cursor = Cursor::Backspace;
        assert_eq!(cursor.get_esc_code(), "\x08 ");

        let cursor = Cursor::Left;
        assert_eq!(cursor.get_esc_code(), "\x1b[1D");

        let cursor = Cursor::Right;
        assert_eq!(cursor.get_esc_code(), "\x1b[1C");

        // let cursor = Cursor::ClearLine;
        // assert_eq!(cursor.get_esc_code(), "\x1b[2K");
    }
}
