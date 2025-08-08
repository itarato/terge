use std::collections::HashMap;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use terge::{
    common::{I32Point, U16Point, intersection_of_rect_and_line},
    line::Line,
    rect::Rect,
};

use crate::text_editor::TextEditor;

macro_rules! action_match {
    ($p:pat, $name:ident) => {
        pub fn $name(&self) -> bool {
            match self {
                $p => true,
                _ => false,
            }
        }
    };
}

macro_rules! action_unwrap {
    ($p:ident, $name:ident, $ret:ty) => {
        pub fn $name(self) -> $ret {
            match self {
                Self::$p(inner) => inner,
                _ => panic!("Failed force unwrap"),
            }
        }
    };
}

/*

enum MyEnum {
    V1(T1),
    V1(T2),
}

impl MyEnum {
fn unwrap_as_v1(&self) -> &T {
    match &self {
        V1(inner) => inner,
        _ => panic!(),
    }
}
}

*/

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
    Freehand,
}

pub struct LineAction {
    pub(crate) start: U16Point,
}
pub struct RectAction {
    pub(crate) start: U16Point,
}
pub struct TextAction {
    pub(crate) start: U16Point,
    pub(crate) editor: TextEditor,
}
pub struct DragRectangleAction {
    pub(crate) rectangle_id: IdType,
    pub(crate) offset: I32Point,
}
pub struct DragLineStartAction {
    pub(crate) line_id: IdType,
}
pub struct DragLineEndAction {
    pub(crate) line_id: IdType,
}
pub struct DragTextAction {
    pub(crate) text_id: IdType,
}
pub struct ResizeRectangleAction {
    pub(crate) rectangle_id: IdType,
    pub(crate) orig_start: U16Point,
}
pub struct FreehandAction {
    pub(crate) points: Vec<U16Point>,
}

pub enum Action {
    Line(LineAction),
    Rect(RectAction),
    Text(TextAction),
    DragRectangle(DragRectangleAction),
    DragLineStart(DragLineStartAction),
    DragLineEnd(DragLineEndAction),
    DragText(DragTextAction),
    ResizeRectangle(ResizeRectangleAction),
    Pointer,
    Freehand(FreehandAction),
}

impl Action {
    // action_match!(Action::Pointer, is_pointer);
    // action_match!(Action::Line(_), is_line);
    // action_match!(Action::Rect(_), is_rect);
    action_match!(Action::Text(_), is_text);
    // action_match!(Action::DragRectangle(_), is_drag_rectangle);
    // action_match!(Action::DragLineStart(_), is_drag_line_start);
    // action_match!(Action::DragLineEnd(_), is_drag_line_end);
    // action_match!(Action::DragText(_), is_drag_text);
    // action_match!(Action::ResizeRectangle(_), is_resize_rectangle);
    // action_match!(Action::Freehand(_), is_freehand);

    action_unwrap!(Line, unwrap_as_line, LineAction);

    action_unwrap!(Rect, unwrap_as_rect, RectAction);
    // action_unwrap!(Text, unwrap_as_text, TextAction);
    // action_unwrap!(DragRectangle, unwrap_as_drag_rectangle, DragRectangleAction);
    action_unwrap!(
        DragLineStart,
        unwrap_as_drag_line_start,
        DragLineStartAction
    );
    action_unwrap!(DragLineEnd, unwrap_as_drag_line_end, DragLineEndAction);
    action_unwrap!(DragText, unwrap_as_drag_text, DragTextAction);
    // action_unwrap!(
    //     ResizeRectangle,
    //     unwrap_as_resize_rectangle,
    //     ResizeRectangleAction
    // );
    action_unwrap!(Freehand, unwrap_as_freehand, FreehandAction);

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
            Action::Freehand { .. } => "freehand",
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
