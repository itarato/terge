use terge::Line;

use crate::common::IdType;

pub struct LineObject {
    pub id: IdType,
    pub line: Line,
    pub start_anchor_rect_id: Option<IdType>,
    pub end_anchor_rect_id: Option<IdType>,
    pub color: usize,
}

impl LineObject {
    pub fn new(id: IdType, line: Line, color: usize) -> Self {
        Self {
            id,
            line,
            start_anchor_rect_id: None,
            end_anchor_rect_id: None,
            color,
        }
    }
}
