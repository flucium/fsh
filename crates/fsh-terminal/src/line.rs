
/// A line of text in the terminal.
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

    // pub(super) fn delete(&mut self) {
    //     self.1.remove(self.0 - 1);
    //     self.0 -= 1;
    // }

    /// Removes a character at the current position.
    pub(super) fn backspace(&mut self) {
        self.1.remove(self.0 - 1);
        self.0 -= 1;
    }

    /// Moves the cursor to the left.
    pub(super) fn move_left(&mut self) {
        self.0 -= 1;
    }

    /// Moves the cursor to the right.
    pub(super) fn move_right(&mut self) {
        self.0 += 1;
    }

    // pub(super) fn clear(&mut self) {
    //     self.0 = 0;
    //     self.1.clear();
    // }

    /// Returns the length of the line.
    pub(super) fn len(&self) -> usize {
        self.1.len()
    }

    /// Returns the position of the cursor.
    pub(super) fn position(&self) -> usize {
        self.0
    }

    // pub(super) fn chars(&self) -> Vec<u8> {
    //     self.1.clone()
    // }
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

        line.backspace();

        assert_eq!(line.0, 4);
        assert_eq!(line.1, vec![b'h', b'e', b'l', b'l']);
    }

    #[test]
    fn test_line_left() {
        let mut line = Line::new();

        line.insert(b'h');
        line.insert(b'e');
        line.insert(b'l');
        line.insert(b'l');
        line.insert(b'o');

        line.move_left();

        assert_eq!(line.0, 4);
    }

    #[test]
    fn test_line_right() {
        let mut line = Line::new();

        line.insert(b'h');
        line.insert(b'e');
        line.insert(b'l');
        line.insert(b'l');
        line.insert(b'o');

        line.move_left();
        line.move_right();

        assert_eq!(line.0, 5);
    }

    // #[test]
    // fn test_line_clear() {
    //     let mut line = Line::new();

    //     line.insert(b'h');
    //     line.insert(b'e');
    //     line.insert(b'l');
    //     line.insert(b'l');
    //     line.insert(b'o');

    //     // line.clear();

    //     assert_eq!(line.0, 0);
    //     assert_eq!(line.1, vec![]);
    // }

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
}
