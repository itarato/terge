use terge::{common::U16Point, gfx::Gfx};

use crate::common::{COLORS, IdType};

pub struct TextObject {
    pub id: IdType,
    pub start: U16Point,
    pub lines: Vec<String>,
    pub anchor_rect_id: Option<IdType>,
    pub color: usize,
}

impl TextObject {
    pub fn new(
        id: IdType,
        start: U16Point,
        lines: Vec<String>,
        anchor_rect_id: Option<IdType>,
        color: usize,
    ) -> Self {
        Self {
            id,
            start,
            lines,
            anchor_rect_id,
            color,
        }
    }

    pub fn is_edit_point(&self, p: U16Point) -> bool {
        self.start == p
    }

    pub fn draw(&self, gfx: &Gfx) {
        if self.anchor_rect_id.is_some() {
            let mid_x = self.start.0;
            let start_y = self.start.1 - (self.lines.len() as u16 / 2);

            for (i, line) in self.lines.iter().enumerate() {
                gfx.draw_text(
                    &line,
                    mid_x - (line.len() / 2) as u16,
                    start_y + i as u16,
                    COLORS[self.color].0,
                );
            }
        } else {
            gfx.draw_multiline_text(
                &self.lines,
                self.start.0,
                self.start.1,
                COLORS[self.color].0,
            );
        }
    }
}
