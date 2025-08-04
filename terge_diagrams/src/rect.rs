use terge::{
    common::{U16Point, point_pair_minmax},
    rect::Rect,
};

use crate::common::IdType;

pub struct RectObject {
    pub id: IdType,
    pub color: usize,
    pub rect: Rect,
}

impl RectObject {
    pub fn new(id: IdType, color: usize, rect: Rect) -> Self {
        Self { id, color, rect }
    }

    pub fn resize(&mut self, previous_start: U16Point, end: U16Point) {
        let (min_x, min_y, max_x, max_y) = point_pair_minmax(previous_start, end);

        self.rect.start = (min_x, min_y);
        self.rect.size = (max_x - min_x, max_y - min_y);
    }
}
