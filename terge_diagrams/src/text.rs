use terge::{
    common::{TextHorizontalAlign, TextVercticalAlign, U16Point, multiline_text_line_start},
    gfx::Gfx,
};

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
        let mut first_checked = false;

        for (i, line) in self.lines.iter().enumerate() {
            let pos = self.line_start(i);

            if !first_checked {
                first_checked = true;
                if pos == p {
                    return false;
                }
            }

            if pos.1 == p.1 && p.0 >= pos.0 && p.0 <= pos.0 + line.len() as u16 {
                return true;
            }
        }

        false
    }

    pub fn draw(&self, gfx: &Gfx) {
        for (i, line) in self.lines.iter().enumerate() {
            let pos = self.line_start(i);
            gfx.draw_text(&line, pos.0, pos.1, COLORS[self.color].0);
        }
    }

    fn line_start(&self, index: usize) -> U16Point {
        let (halign, valign) = if self.anchor_rect_id.is_some() {
            (TextHorizontalAlign::Center, TextVercticalAlign::Center)
        } else {
            (TextHorizontalAlign::Left, TextVercticalAlign::Top)
        };

        multiline_text_line_start(
            self.lines.len() as u16,
            self.lines[index].len() as u16,
            index as u16,
            self.start,
            halign,
            valign,
        )
    }
}
