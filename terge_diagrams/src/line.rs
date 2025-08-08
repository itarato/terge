use terge::{common::U16Point, line::Line};

use crate::common::IdType;

pub struct LineObject {
    pub id: IdType,
    pub line: Line,
    pub start_anchor_rect_id: Option<IdType>,
    pub end_anchor_rect_id: Option<IdType>,
    pub color: usize,
    pub segment: Option<U16Point>,
}

impl LineObject {
    pub fn new(id: IdType, line: Line, color: usize) -> Self {
        Self {
            id,
            line,
            start_anchor_rect_id: None,
            end_anchor_rect_id: None,
            color,
            segment: None,
        }
    }

    pub(crate) fn is_drag_point(&self, p: U16Point) -> bool {
        p == self.line.start || p == self.line.end
    }

    pub(crate) fn is_point_on(&self, p: U16Point) -> bool {
        if let Some(segment) = self.segment {
            Line {
                start: self.line.start,
                end: segment,
            }
            .is_point_on(p)
                || Line {
                    start: segment,
                    end: self.line.end,
                }
                .is_point_on(p)
        } else {
            self.line.is_point_on(p)
        }
    }

    pub(crate) fn start_line_segment(&self) -> Line {
        if let Some(segment) = self.segment {
            Line {
                start: self.line.start,
                end: segment,
            }
        } else {
            self.line
        }
    }

    pub(crate) fn end_line_segment(&self) -> Line {
        if let Some(segment) = self.segment {
            Line {
                start: segment,
                end: self.line.end,
            }
        } else {
            self.line
        }
    }
}
