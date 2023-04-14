pub struct Editor {
    column: usize,
    row: usize,
    lines: Vec<String>,
}

pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Editor {
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
        // We can retrieve lines unchecked here because we know our position and how many lines
        // there are and as long as we enter with a sane state, we should exit with one as well
        unsafe {
            match direction {
                Direction::Up => {
                    if self.row > 0 {
                        self.row -= 1;
                        let line_length = self.lines.get_unchecked(self.row).len();
                        if self.column > line_length {
                            self.column = line_length;
                        }
                    } else {
                        self.column = 0;
                    }
                }
                Direction::Right => {
                    let line_length = self.lines.get_unchecked(self.row).len();
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
                        let line_length = self.lines.get_unchecked(self.row).len();
                        if self.column > line_length {
                            self.column = line_length;
                        }
                    } else {
                        let line_length = self.lines.get_unchecked(self.row).len();
                        self.column = line_length;
                    }
                }
                Direction::Left => {
                    if self.column > 0 {
                        self.column -= 1;
                    } else if self.row > 0 {
                        self.row -= 1;
                        let line_length = self.lines.get_unchecked(self.row).len();
                        self.column = line_length;
                    }
                }
            }
        }
    }

    pub fn position(&self) -> (usize, usize) {
        (self.row, self.column)
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
}
