use std::collections::HashMap;

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
pub(crate) const POINTER_STR: &'static str = "*";
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
    Pointer,
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
    DragText {
        text_id: IdType,
    },
    ResizeRectangle {
        rectangle_id: IdType,
        orig_start: U16Point,
    },
    Pointer,
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
            Action::DragText { .. } => "drag text",
            Action::Pointer => "pointer",
        }
    }
}

pub struct PointerPoint {
    pub pos: U16Point,
    pub deadline: u128,
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

pub fn delete_one_from_list_with_cond<K, V, C>(list: &mut HashMap<K, V>, cond: C) -> bool
where
    C: Fn(&mut V) -> bool,
{
    let mut done = false;

    list.retain(|_, o| {
        if !done && cond(o) {
            done = true;
            false
        } else {
            true
        }
    });

    done
}

pub(crate) fn get_current_time_ms() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

pub const CLICK_TRACE_MAP: [[i8; 2]; 16] = [
    [-2, -2],
    [-1, -2],
    [0, -2],
    [1, -2],
    [2, -2],
    [2, -1],
    [2, 0],
    [2, 1],
    [2, 2],
    [1, 2],
    [0, 2],
    [-1, 2],
    [-2, 2],
    [-2, 1],
    [-2, 0],
    [-2, -1],
];

pub const CLICK_TRACE_STRS: [&'static str; 5] = [" ", "░", "▒", "▓", "▉"];
