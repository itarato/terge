use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use terge::{
    common::{I32Point, U16Point, intersection_of_rect_and_line},
    line::Line,
    rect::Rect,
};

use crate::text_editor::TextEditor;

pub(crate) type IdType = u64;

pub(crate) const DRAG_STR: &'static str = "+";
pub(crate) const EDIT_STR: &'static str = "#";
pub(crate) const COLORS: [(u8, &'static str); 10] = [
    (39, "Default color"),
    (31, "Red"),
    (33, "Yellow"),
    (90, "Dark gray"),
    (91, "Light red"),
    (92, "Light green"),
    (93, "Light yellow"),
    (94, "Light blue"),
    (95, "Light magenta"),
    (96, "Light cyan"),
];
pub(crate) const DEFAULT_COLOR_CODE: u8 = COLORS[0].0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Intent {
    Line,
    Rect,
    Text,
}

pub enum Action {
    Line {
        start: U16Point,
    },
    Rect {
        start: U16Point,
    },
    Text {
        start: U16Point,
        editor: TextEditor,
    },
    DragRectangle {
        rectangle_id: IdType,
        offset: I32Point,
    },
    DragLineStart {
        line_id: IdType,
    },
    DragLineEnd {
        line_id: IdType,
    },
    ResizeRectangle {
        rectangle_id: IdType,
        orig_start: U16Point,
    },
}

impl Action {
    pub fn is_text(&self) -> bool {
        match self {
            Action::Text { .. } => true,
            _ => false,
        }
    }

    pub fn to_string_short(&self) -> &str {
        match self {
            Action::Line { .. } => "line",
            Action::Rect { .. } => "rect",
            Action::Text { .. } => "text",
            Action::DragRectangle { .. } => "drag rectangle",
            Action::ResizeRectangle { .. } => "resize rectangle",
            Action::DragLineStart { .. } => "drag line start",
            Action::DragLineEnd { .. } => "drag line end",
        }
    }
}

pub fn intersection_of_rect_and_anchored_line(rect: &Rect, line: &Line) -> Option<U16Point> {
    let intersections = intersection_of_rect_and_line(rect, line);

    for p in intersections {
        if line.x_range().contains(&p.0) && line.y_range().contains(&p.1) {
            return Some(p);
        }
    }

    None
}

pub(crate) trait KeyEventUtil {
    fn is_enter_without_alt(&self) -> bool;
}

impl KeyEventUtil for KeyEvent {
    fn is_enter_without_alt(&self) -> bool {
        self.code == KeyCode::Enter && !self.modifiers.contains(KeyModifiers::ALT)
    }
}
