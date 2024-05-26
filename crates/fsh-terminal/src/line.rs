/// A Line obj of text in the terminal.
pub(super) struct Line(usize, Vec<u8>);

impl Line {
    /// Creates a new line.
    pub(super) const fn new() -> Self {
        Line(0, Vec::new())
    }

    /// Inserts a character at the current position.
    pub(super) fn insert(&mut self, c: u8) {
        self.1.insert(self.0, c);
        self.0 += 1;
    }

    /// Removes a character at the current position.
    pub(super) fn backspace(&mut self) {
        self.1.remove(self.0 - 1);
        self.0 -= 1;
    }

    /// Moves the cursor to the left.
    pub(super) fn move_left(&mut self) {
        if self.0 > 0 {
            self.0 -= 1;
        }
    }

    /// Moves the cursor to the right.
    pub(super) fn move_right(&mut self) {
        if self.0 < self.len() {
            self.0 += 1;
        }
    }

    /// Returns the length of the line.
    pub(super) fn len(&self) -> usize {
        self.1.len()
    }

    /// Returns the position of the cursor.
    pub(super) fn position(&self) -> usize {
        self.0
    }
}

impl ToString for Line {
    /// Converts the line to a string.
    fn to_string(&self) -> String {
        String::from_utf8_lossy(&self.1).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_new() {
        let line = Line::new();

        assert_eq!(line.0, 0);
        assert_eq!(line.1, vec![]);
    }

    #[test]
    fn test_line_len() {
        let mut line = Line::new();

        line.insert(b'h');
        line.insert(b'e');
        line.insert(b'l');
        line.insert(b'l');
        line.insert(b'o');

        assert_eq!(line.len(), 5);
    }

    #[test]
    fn test_line_position() {
        let mut line = Line::new();

        line.insert(b'h');
        line.insert(b'e');
        line.insert(b'l');
        line.insert(b'l');
        line.insert(b'o');

        assert_eq!(line.position(), 5);
    }

    #[test]
    fn test_to_string() {
        let mut line = Line::new();

        line.insert(b'h');
        line.insert(b'e');
        line.insert(b'l');
        line.insert(b'l');
        line.insert(b'o');

        assert_eq!(line.to_string(), "hello");
    }

    #[test]
    fn test_line_insert() {
        let mut line = Line::new();

        line.insert(b'h');
        line.insert(b'e');
        line.insert(b'l');
        line.insert(b'l');
        line.insert(b'o');

        assert_eq!(line.0, 5);
        assert_eq!(line.1, vec![b'h', b'e', b'l', b'l', b'o']);
    }

    #[test]
    fn test_line_backspace() {
        let mut line = Line::new();

        line.insert(b'h');
        line.insert(b'e');
        line.insert(b'l');
        line.insert(b'l');
        line.insert(b'o');

        // Remove the last character.
        line.backspace();
        assert_eq!(line.0, 4);
        assert_eq!(line.1, vec![b'h', b'e', b'l', b'l']);

        // Insert a character.
        line.insert(b'o');
        assert_eq!(line.0, 5);
        assert_eq!(line.1, vec![b'h', b'e', b'l', b'l', b'o']);

        // Backspace 4 characters.
        for _ in 0..4 {
            line.backspace();
        }
        assert_eq!(line.0, 1);

        // Insert a character.
        line.insert(b'e');
        line.insert(b'l');
        line.insert(b'l');
        line.insert(b'o');
        assert_eq!(line.0, 5);
        assert_eq!(line.1, vec![b'h', b'e', b'l', b'l', b'o']);
    }

    #[test]
    fn test_line_left() {
        let mut line = Line::new();

        line.insert(b'h');
        line.insert(b'e');
        line.insert(b'l');
        line.insert(b'l');
        line.insert(b'o');

        // Move the cursor to the left.
        // Check if the cursor is at the correct position.
        line.move_left();
        assert_eq!(line.0, 4);

        // Move the cursor to the left.
        // Check if the cursor is at the correct position.
        for _ in 0..4 {
            line.move_left();
        }
        assert_eq!(line.0, 0);

        // Move the cursor to the left.
        // Check if the cursor is at the correct position.
        line.move_left();
        assert_eq!(line.0, 0);
    }

    #[test]
    fn test_line_right() {
        let mut line = Line::new();

        line.insert(b'h');
        line.insert(b'e');
        line.insert(b'l');
        line.insert(b'l');
        line.insert(b'o');

        // Move the cursor to the right.
        // Check if the cursor is at the correct position.
        line.move_right();
        assert_eq!(line.0, 5);

        // Move the cursor to the right.
        // Check if the cursor is at the correct position.
        for _ in 0..100 {
            line.move_right();
        }
        assert_eq!(line.0, 5);

        // Move the cursor to the left.
        // Check if the cursor is at the correct position.
        for _ in 0..100 {
            line.move_left();
        }
        assert_eq!(line.0, 0);

        // Move the cursor to the right.
        // Check if the cursor is at the correct position.
        line.move_right();
        assert_eq!(line.0, 1);
    }
}
