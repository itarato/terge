use std::io::{self, Write};

use crossterm::{ExecutableCommand, terminal};

use crate::common::*;
use crate::line::{Line, LinePointsIterator};
use crate::rect::Rect;

pub struct Gfx {
    pub width: u16,
    pub height: u16,
}

impl Gfx {
    pub(crate) fn new() -> Self {
        Self {
            width: 0,
            height: 0,
        }
    }

    pub(crate) fn refresh_state(&mut self) {
        let size = crossterm::terminal::size().expect("Failed getting size");

        self.width = size.0;
        self.height = size.1;
    }

    pub fn clear_screen(&self) {
        io::stdout()
            .execute(terminal::Clear(terminal::ClearType::All))
            .expect("Failed cleaning terminal");
        self.flush_buffer();
    }

    fn draw_pos(&self, x: u16, y: u16) {
        io::stdout()
            .execute(crossterm::cursor::MoveTo(x % self.width, y % self.height))
            .expect("Failed moving cursor position");
    }

    pub fn draw_text(&self, text: &str, x: u16, y: u16, color: u8) {
        self.draw_pos(x, y);
        print!("\x1B[{}m{}\x1B[0m", color, text);
    }

    pub fn draw_text_uncoloured(&self, text: &str, x: u16, y: u16) {
        self.draw_pos(x, y);
        io::stdout()
            .write_all(text.as_bytes())
            .expect("Failed writing bytes");
    }

    pub fn draw_text_to_current_pos(&self, text: &str) {
        print!("{}", text);
    }

    pub fn draw_text_at_point(&self, text: &str, p: U16Point, color: u8) {
        self.draw_text(text, p.0, p.1, color);
    }

    pub fn draw_multiline_text(&self, lines: &Vec<String>, x: u16, y: u16, color: u8) {
        for (i, line) in lines.iter().enumerate() {
            self.draw_text(&line, x, y + i as u16, color);
        }
    }

    pub(crate) fn flush_buffer(&self) {
        std::io::stdout().flush().expect("Failed flushing STDOUT");
    }

    pub fn draw_rect(&self, rect: &Rect, color_code: u8) {
        self.draw_rect_from_points(rect.start, rect.start.add(rect.size), color_code);
    }

    pub fn draw_rect_from_points(&self, lhs: U16Point, rhs: U16Point, color_code: u8) {
        let (x_min, y_min, x_max, y_max) = point_pair_minmax(lhs, rhs);

        self.draw_text_to_current_pos(&format!("\x1B[{}m", color_code));

        for y in y_min..=y_max {
            self.draw_text_uncoloured(BOX_VERTICAL_CHAR, x_min, y);
            self.draw_text_uncoloured(BOX_VERTICAL_CHAR, x_max, y);
        }

        if x_max - x_min >= 2 {
            self.draw_text_uncoloured(
                &BOX_HORIZONTAL_CHAR.repeat((x_max - x_min - 1) as usize),
                x_min + 1,
                y_min,
            );
            self.draw_text_uncoloured(
                &BOX_HORIZONTAL_CHAR.repeat((x_max - x_min - 1) as usize),
                x_min + 1,
                y_max,
            );
        }

        self.draw_text_uncoloured(BOX_TOP_LEFT_CORNER_CHAR, x_min, y_min);
        self.draw_text_uncoloured(BOX_TOP_RIGTH_CORNER_CHAR, x_max, y_min);
        self.draw_text_uncoloured(BOX_BOTTOM_LEFT_CORNER_CHAR, x_min, y_max);
        self.draw_text_uncoloured(BOX_BOTTOM_RIGTH_CORNER_CHAR, x_max, y_max);

        self.draw_text_to_current_pos("\x1B[0m");
    }

    pub fn draw_line(&self, line: &Line, color: u8) {
        self.draw_line_from_points(line.start, line.end, color);
    }

    pub fn draw_line_from_points(&self, start: U16Point, end: U16Point, color: u8) {
        self.draw_text_to_current_pos(&format!("\x1B[{}m", color));

        for (x, y) in LinePointsIterator::new(start, end) {
            self.draw_text_uncoloured(BLOCK_CHAR, x, y);
        }

        self.draw_text_uncoloured(LINE_CONNECTION_CHAR, start.0, start.1);
        self.draw_text_uncoloured(LINE_CONNECTION_CHAR, end.0, end.1);

        self.draw_text_to_current_pos("\x1B[0m");
    }
}
