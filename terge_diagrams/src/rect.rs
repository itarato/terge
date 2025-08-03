use terge::{Arithmetics, I32Point, Rect, point_pair_minmax};

use crate::common::IdType;

// TODO: Add hover-over highlight
pub struct RectObject {
    pub id: IdType,
    pub color: usize,
    pub rect: Rect,
}

impl RectObject {
    pub fn new(id: IdType, color: usize, rect: Rect) -> Self {
        Self { id, color, rect }
    }

    pub fn end(&self) -> I32Point {
        self.rect.start.add(self.rect.size)
    }

    pub fn resize(&mut self, previous_start: I32Point, end: I32Point) {
        let (min_x, min_y, max_x, max_y) = point_pair_minmax(previous_start, end);

        self.rect.start = (min_x, min_y);
        self.rect.size = (max_x - min_x, max_y - min_y);
    }
}
