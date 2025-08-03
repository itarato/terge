use std::io::{self, Write};

use crossterm::{ExecutableCommand, terminal};

use crate::common::*;
use crate::line::{Line, LinePointsIterator};
use crate::rect::Rect;

pub struct Gfx {
    pub width: usize,
    pub height: usize,
}

impl Gfx {
    pub(crate) fn new() -> Self {
        Self {
            width: 0,
            height: 0,
        }
    }

    pub(crate) fn refresh_state(&mut self) {
        if let Some((w, h)) = term_size::dimensions() {
            self.width = w;
            self.height = h;
        }
    }

    pub fn clear_screen(&self) {
        io::stdout()
            .execute(terminal::Clear(terminal::ClearType::All))
            .expect("Failed cleaning terminal");
        self.flush_buffer();
    }

    fn draw_pos(&self, x: usize, y: usize) {
        print!("\x1B[{};{}H", y + 1, x + 1);
    }

    pub fn draw_text(&self, text: &str, x: usize, y: usize, color: u8) {
        self.draw_pos(x, y);
        print!("\x1B[{}m{}\x1B[0m", color, text);
    }

    pub fn draw_text_uncoloured(&self, text: &str, x: usize, y: usize) {
        self.draw_pos(x, y);
        io::stdout()
            .write_all(text.as_bytes())
            .expect("Failed writing bytes");
    }

    pub fn draw_text_to_current_pos(&self, text: &str) {
        print!("{}", text);
    }

    pub fn draw_text_at_point(&self, text: &str, p: I32Point, color: u8) {
        self.draw_text(text, p.0 as usize, p.1 as usize, color);
    }

    pub fn draw_multiline_text(&self, lines: &Vec<String>, x: usize, y: usize, color: u8) {
        for (i, line) in lines.iter().enumerate() {
            self.draw_text(&line, x, y + i, color);
        }
    }

    pub(crate) fn flush_buffer(&self) {
        std::io::stdout().flush().expect("Failed flushing STDOUT");
    }

    pub fn draw_rect(&self, rect: &Rect, color_code: u8) {
        self.draw_rect_from_points(rect.start, rect.start.add(rect.size), color_code);
    }

    pub fn draw_rect_from_points(&self, lhs: I32Point, rhs: I32Point, color_code: u8) {
        let (x_min, y_min, x_max, y_max) = point_pair_minmax(lhs, rhs);

        self.draw_text_to_current_pos(&format!("\x1B[{}m", color_code));

        for y in y_min..=y_max {
            self.draw_text_uncoloured(BLOCK_CHAR, x_min as usize, y as usize);
            self.draw_text_uncoloured(BLOCK_CHAR, x_max as usize, y as usize);
        }
        self.draw_text_uncoloured(
            &BLOCK_CHAR.repeat((x_max - x_min) as usize),
            x_min as usize,
            y_min as usize,
        );
        self.draw_text_uncoloured(
            &BLOCK_CHAR.repeat((x_max - x_min) as usize),
            x_min as usize,
            y_max as usize,
        );

        self.draw_text_to_current_pos("\x1B[0m");
    }

    pub fn draw_line(&self, line: &Line, color: u8) {
        self.draw_line_from_points(line.start, line.end, color);
    }

    pub fn draw_line_from_points(&self, start: I32Point, end: I32Point, color: u8) {
        self.draw_text_to_current_pos(&format!("\x1B[{}m", color));

        for (x, y) in LinePointsIterator::new(start, end) {
            self.draw_text_uncoloured(BLOCK_CHAR, x as usize, y as usize);
        }

        self.draw_text_to_current_pos("\x1B[0m");
    }
}
