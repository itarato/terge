use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use terge::common::UsizePoint;

pub struct TextEditor {
    cursor: UsizePoint,
    pub lines: Vec<String>,
}

impl TextEditor {
    pub fn new() -> Self {
        Self {
            cursor: (0, 0), // X is ignored for now.
            lines: vec![String::new()],
        }
    }

    pub fn new_with_lines(lines: Vec<String>) -> Self {
        Self {
            cursor: (0, lines.len() - 1),
            lines,
        }
    }

    pub fn edit(&mut self, event: &KeyEvent) {
        match event.code {
            KeyCode::Char(c) => {
                self.lines[self.cursor.1].push(c);
            }
            KeyCode::Backspace => {
                if self.lines[self.cursor.1].pop().is_none() {
                    if self.cursor.1 > 0 {
                        self.lines.remove(self.cursor.1);
                        self.cursor.1 -= 1;
                    }
                }
            }
            KeyCode::Enter => {
                if event.modifiers.contains(KeyModifiers::ALT) {
                    self.cursor.1 += 1;
                    self.lines.insert(self.cursor.1, String::new());
                }
            }
            _ => {}
        }
    }
}
