pub struct Editor {
    column: usize,
    row: usize,
    lines: Vec<String>,
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Editor {
    #[must_use]
    pub fn new() -> Self {
        Self {
            column: 0,
            row: 0,
            lines: Vec::new(),
        }
    }

    pub fn insert(&mut self, ch: char) {
        if let Some(line) = self.lines.get_mut(self.row) {
            line.insert(self.column, ch);
        } else {
            self.lines.push(String::from(ch));
        }
        self.column += 1;
    }

    pub fn insert_str(&mut self, str: &str) {
        for (index, insert) in str.split('\n').enumerate() {
            if index > 0 {
                self.row += 1;
                self.column = 0;
            }
            if let Some(line) = self.lines.get_mut(self.row) {
                line.insert_str(self.column, insert);
            } else {
                self.lines.push(String::from(insert));
            }
            self.column += insert.len();
        }
    }

    pub fn move_cursor(&mut self, direction: Direction) {
        match direction {
            Direction::Up => {
                if self.row > 0 {
                    self.row -= 1;
                    let line_length = self.get_line().len();
                    if self.column > line_length {
                        self.column = line_length;
                    }
                } else {
                    self.column = 0;
                }
            }
            Direction::Right => {
                let line_length = self.get_line().len();
                if self.column < line_length {
                    self.column += 1;
                } else if self.column == line_length && self.row < self.lines.len() - 1 {
                    self.row += 1;
                    self.column = 0;
                }
            }
            Direction::Down => {
                if self.row < self.lines.len() - 1 {
                    self.row += 1;
                    let line_length = self.get_line().len();
                    if self.column > line_length {
                        self.column = line_length;
                    }
                } else {
                    let line_length = self.get_line().len();
                    self.column = line_length;
                }
            }
            Direction::Left => {
                if self.column > 0 {
                    self.column -= 1;
                } else if self.row > 0 {
                    self.row -= 1;
                    let line_length = self.get_line().len();
                    self.column = line_length;
                }
            }
        }
    }

    #[must_use]
    pub fn position(&self) -> (usize, usize) {
        (self.row, self.column)
    }

    pub fn remove_char(&mut self) {
        if self.position() == (0,0) { 
            return;
        }

        if self.column == 0 {
            let current_line = self.get_line().to_owned();
            let line_above = unsafe { self.lines.get_unchecked_mut(self.row - 1) };

            line_above.push_str(&current_line);
            self.lines.remove(self.row);
            self.row -= 1;
            let line_length = self.get_line().len();
            if self.column > line_length {
                self.column = line_length;
            }
        } else {
            self.column -= 1;
            let current_line = unsafe { self.lines.get_unchecked_mut(self.row) };
            current_line.remove(self.column);
        }
    }

    #[must_use]
    pub fn get_line(&self) -> &str {
        unsafe { self.lines.get_unchecked(self.row) }
    }

    #[must_use]
    pub fn get_line_mut(&mut self) -> &mut String {
        unsafe { self.lines.get_unchecked_mut(self.row) }
    }
}

impl Default for Editor {
    fn default() -> Self {
        Self::new()
    }
}

impl ToString for Editor {
    fn to_string(&self) -> String {
        self.lines.join("\n")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn insert() {
        let mut editor = Editor::new();
        let text = "Hello World!";

        for ch in text.chars() {
            editor.insert(ch);
        }

        assert_eq!(editor.to_string(), text);
    }

    #[test]
    fn insert_str() {
        let mut editor = Editor::new();
        let text = "Hello World!";

        editor.insert_str(text);

        assert_eq!(editor.to_string(), text);
    }

    #[test]
    fn insert_str_with_newline() {
        let mut editor = Editor::new();
        let text = "Hello\nwonderful\nWorld!";

        editor.insert_str(text);

        assert!(editor.lines.len() == 3);
        assert_eq!(editor.to_string(), text);
    }

    #[test]
    fn check_movement() {
        let editor = &mut Editor::new();
        let text = "Short Line\n===Long Line===\n===Long Line==";

        editor.insert_str(text);
        move_and_check(editor, Direction::Up, (1, 14));
        // This should not only move us up, but also to the end of the line
        move_and_check(editor, Direction::Up, (0, 10));
        // If we go right, we end up at the start of the next line
        move_and_check(editor, Direction::Right, (1, 0));
        // And we go back
        move_and_check(editor, Direction::Left, (0, 10));
        // Going up moves us to the start of the line
        move_and_check(editor, Direction::Up, (0, 0));
        // Going left should do nothing
        move_and_check(editor, Direction::Left, (0, 0));
        // Now we go down two
        move_and_check(editor, Direction::Down, (1, 0));
        move_and_check(editor, Direction::Down, (2, 0));
        // And going down should bring us to the end again
        move_and_check(editor, Direction::Down, (2, 14));
        // Going right should do nothing
        move_and_check(editor, Direction::Right, (2, 14));
    }

    fn move_and_check(editor: &mut Editor, direction: Direction, position: (usize, usize)) {
        editor.move_cursor(direction);
        assert_eq!(editor.position(), position);
    }

    #[test]
    fn remove_char() {
        let mut editor = Editor::new();

        // We insert the text, putting our cursor at the end.
        editor.insert_str("Hello\nWorld!");
        editor.remove_char();
        // This should have removed the exclamation mark.
        assert_eq!(editor.to_string(), "Hello\nWorld");
        editor.move_cursor(Direction::Up);
        editor.move_cursor(Direction::Right);
        editor.remove_char();
        // We moved to the start of 'World' and removed a character, making the entire text one line.
        assert_eq!(editor.to_string(), "HelloWorld");
        editor.move_cursor(Direction::Up);
        editor.remove_char();
        // We moved to the start of the document and tried to remove a character. Nothing should have happened.
        assert_eq!(editor.to_string(), "HelloWorld");
    }
}
