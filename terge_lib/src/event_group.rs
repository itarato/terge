use crossterm::event::{Event, KeyCode, KeyEvent};

#[derive(Debug, Default)]
pub struct EventGroup {
    pub events: Vec<Event>,
}

impl EventGroup {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub fn first_pressed_char(&self) -> Option<char> {
        for e in &self.events {
            match e {
                Event::Key(KeyEvent {
                    code: KeyCode::Char(c),
                    ..
                }) => return Some(*c),
                _ => {}
            }
        }
        None
    }

    pub fn did_press_key(&self, key_code: KeyCode) -> bool {
        for e in &self.events {
            match e {
                Event::Key(key_event) => {
                    if key_event.code == key_code {
                        return true;
                    }
                }
                _ => {}
            }
        }
        false
    }

    pub fn last_mouse_pos(&self) -> Option<(u16, u16)> {
        for e in self.events.iter().rev() {
            match e {
                Event::Mouse(mouse_event) => return Some((mouse_event.column, mouse_event.row)),
                _ => {}
            }
        }
        None
    }
}
