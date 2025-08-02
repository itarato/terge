use std::{
    io::{self, Write},
    ops,
    sync::{Arc, atomic::AtomicBool, mpsc},
    thread,
    time::Duration,
};

use crossterm::{
    ExecutableCommand, cursor,
    event::{self, Event, KeyCode, KeyEvent, poll, read},
    terminal,
};
use log::trace;

pub const BLOCK_CHAR: &'static str = "█";

pub type I32Point = (i32, i32);
pub type UsizePoint = (usize, usize);

pub trait Arithmetics<T> {
    fn add(&self, other: T) -> T;
    fn sub(&self, other: T) -> T;
    fn div(&self, divisor: i32) -> T;
}

impl Arithmetics<I32Point> for I32Point {
    fn add(&self, other: I32Point) -> I32Point {
        (self.0 + other.0, self.1 + other.1)
    }

    fn sub(&self, other: I32Point) -> I32Point {
        (self.0 - other.0, self.1 - other.1)
    }

    fn div(&self, divisor: i32) -> I32Point {
        (self.0 / divisor, self.1 / divisor)
    }
}

pub fn point_pair_minmax(lhs: I32Point, rhs: I32Point) -> (i32, i32, i32, i32) {
    (
        lhs.0.min(rhs.0),
        lhs.1.min(rhs.1),
        lhs.0.max(rhs.0),
        lhs.1.max(rhs.1),
    )
}

pub fn intersection_of_rect_and_line(rect: &Rect, line: &Line) -> Vec<I32Point> {
    let slope = line.slope();
    let mut out = vec![];

    if line.y_range().contains(&rect.start.1) {
        let intersect_top_x =
            (((rect.start.1 - line.start.1) as f32 / slope) + line.start.0 as f32).round() as i32;
        if intersect_top_x >= rect.start.0 && intersect_top_x <= rect.start.0 + rect.size.0 {
            out.push((intersect_top_x, rect.start.1));
        }
    }

    if line.y_range().contains(&(rect.start.1 + rect.size.1)) {
        let intersect_bottom_x = (((rect.start.1 + rect.size.1 - line.start.1) as f32 / slope)
            + line.start.0 as f32)
            .round() as i32;
        if intersect_bottom_x >= rect.start.0 && intersect_bottom_x <= rect.start.0 + rect.size.0 {
            out.push((intersect_bottom_x, rect.start.1 + rect.size.1));
        }
    }

    if line.x_range().contains(&rect.start.0) {
        let intersect_left_y =
            ((slope * (rect.start.0 - line.start.0) as f32) + line.start.1 as f32).round() as i32;
        if intersect_left_y >= rect.start.1 && intersect_left_y <= rect.start.1 + rect.size.1 {
            out.push((rect.start.0, intersect_left_y));
        }
    }

    if line.x_range().contains(&(rect.start.0 + rect.size.0)) {
        let intersect_right_y = ((slope * (rect.start.0 + rect.size.0 - line.start.0) as f32)
            + line.start.1 as f32)
            .round() as i32;
        if intersect_right_y >= rect.start.1 && intersect_right_y <= rect.start.1 + rect.size.1 {
            out.push((rect.start.0 + rect.size.0, intersect_right_y));
        }
    }

    out
}

pub struct Rect {
    // Start is always the upper-left corner (min-x:min-y).
    pub start: I32Point,
    // Size is always positive.
    pub size: I32Point,
}

impl Rect {
    pub fn new_from_unordered_points(lhs: I32Point, rhs: I32Point) -> Self {
        let (min_x, min_y, max_x, max_y) = point_pair_minmax(lhs, rhs);
        Self {
            start: (min_x, min_y),
            size: (max_x - min_x, max_y - min_y),
        }
    }

    pub fn is_point_on_header(&self, p: I32Point) -> bool {
        p.1 == self.start.1 && p.0 >= self.start.0 && p.0 <= (self.start.0 + self.size.0)
    }

    pub fn is_point_on(&self, p: I32Point) -> bool {
        p.0 >= self.start.0
            && p.0 <= (self.start.0 + self.size.0)
            && p.1 >= self.start.1
            && p.1 <= (self.start.1 + self.size.1)
    }

    pub fn is_point_inside(&self, p: I32Point) -> bool {
        p.0 >= self.start.0 + 1
            && p.0 <= (self.start.0 + self.size.0) - 1
            && p.1 >= self.start.1 + 1
            && p.1 <= (self.start.1 + self.size.1) - 1
    }

    pub fn midpoint(&self) -> I32Point {
        self.start.add(self.size.div(2))
    }
}

#[derive(Debug)]
pub struct Line {
    pub start: I32Point,
    pub end: I32Point,
}

impl Line {
    fn slope(&self) -> f32 {
        let dx = self.end.0 - self.start.0;
        let dy = self.end.1 - self.start.1;
        dy as f32 / dx as f32
    }

    pub fn x_range(&self) -> ops::RangeInclusive<i32> {
        self.start.0.min(self.end.0)..=self.start.0.max(self.end.0)
    }

    pub fn y_range(&self) -> ops::RangeInclusive<i32> {
        self.start.1.min(self.end.1)..=self.start.1.max(self.end.1)
    }
}

pub trait App {
    fn reset(&mut self, gfx: &mut Gfx);
    fn update(&mut self, events: &EventGroup, gfx: &mut Gfx) -> bool;
    fn draw(&self, gfx: &mut Gfx);
}

#[derive(Debug, Default)]
pub struct EventGroup {
    pub events: Vec<Event>,
}

impl EventGroup {
    fn new() -> Self {
        Self::default()
    }

    pub fn first_pressed_char(&self) -> Option<char> {
        for e in &self.events {
            match e {
                Event::Key(KeyEvent {
                    code: KeyCode::Char(c),
                    ..
                }) => return Some(*c),
                _ => {}
            }
        }
        None
    }

    pub fn did_press_key(&self, key_code: KeyCode) -> bool {
        for e in &self.events {
            match e {
                Event::Key(key_event) => {
                    if key_event.code == key_code {
                        return true;
                    }
                }
                _ => {}
            }
        }
        false
    }

    pub fn last_mouse_pos(&self) -> Option<(u16, u16)> {
        for e in self.events.iter().rev() {
            match e {
                Event::Mouse(mouse_event) => return Some((mouse_event.column, mouse_event.row)),
                _ => {}
            }
        }
        None
    }
}

pub struct Gfx {
    pub width: usize,
    pub height: usize,
}

impl Gfx {
    fn new() -> Self {
        Self {
            width: 0,
            height: 0,
        }
    }

    fn refresh_state(&mut self) {
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

    pub fn draw_text(&self, text: &str, x: usize, y: usize) {
        self.draw_pos(x, y);
        print!("{}", text);
    }

    pub fn draw_text_at_point(&self, text: &str, p: I32Point) {
        self.draw_text(text, p.0 as usize, p.1 as usize);
    }

    pub fn draw_multiline_text(&self, lines: &Vec<String>, x: usize, y: usize) {
        for (i, line) in lines.iter().enumerate() {
            self.draw_text(&line, x, y + i);
        }
    }

    fn flush_buffer(&self) {
        std::io::stdout().flush().expect("Failed flushing STDOUT");
    }

    pub fn draw_rect(&self, rect: &Rect) {
        self.draw_rect_from_points(rect.start, rect.start.add(rect.size));
    }

    pub fn draw_rect_from_points(&self, lhs: I32Point, rhs: I32Point) {
        let (x_min, y_min, x_max, y_max) = point_pair_minmax(lhs, rhs);

        for y in y_min..=y_max {
            self.draw_text(BLOCK_CHAR, x_min as usize, y as usize);
            self.draw_text(BLOCK_CHAR, x_max as usize, y as usize);
        }
        self.draw_text(
            &BLOCK_CHAR.repeat((x_max - x_min) as usize),
            x_min as usize,
            y_min as usize,
        );
        self.draw_text(
            &BLOCK_CHAR.repeat((x_max - x_min) as usize),
            x_min as usize,
            y_max as usize,
        );
    }

    pub fn draw_line(&self, line: &Line) {
        self.draw_line_from_points(line.start, line.end);
    }

    pub fn draw_line_from_points(&self, start: I32Point, end: I32Point) {
        let (x_min, y_min, x_max, y_max) = point_pair_minmax(start, end);

        let diff_x = (end.0 - start.0) as f32;
        let diff_y = (end.1 - start.1) as f32;
        let diff_x_abs = diff_x.abs();
        let diff_y_abs = diff_y.abs();

        if diff_x_abs >= diff_y_abs {
            if diff_x != 0.0 {
                for x in x_min..=x_max {
                    let y = ((diff_y / diff_x) * (x as f32 - start.0 as f32) + start.1 as f32)
                        .round() as usize;
                    self.draw_text(BLOCK_CHAR, x as usize, y);
                }
            }
        } else {
            if diff_y != 0.0 {
                for y in y_min..=y_max {
                    let x = ((diff_x / diff_y) * (y as f32 - start.1 as f32) + start.0 as f32)
                        .round() as usize;
                    self.draw_text(BLOCK_CHAR, x, y as usize);
                }
            }
        }
    }
}

fn get_current_ms() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

pub struct Terge {
    app: Box<dyn App>,
    gfx: Gfx,
    target_frame_length_ms: u128,
    should_terminate: bool,
}

impl Terge {
    pub fn new(app: Box<dyn App>) -> Self {
        Self {
            app,
            gfx: Gfx::new(),
            target_frame_length_ms: 16,
            should_terminate: false,
        }
    }

    fn turn_on_terminal_raw_mode(&self) {
        crossterm::terminal::enable_raw_mode().expect("Failed to enable raw mode");
        io::stdout()
            .execute(cursor::Hide)
            .expect("Failed running crossterm commands");
        io::stdout()
            .execute(event::EnableMouseCapture)
            .expect("Failed enabling mouse capture");
    }

    fn turn_off_terminal_raw_mode(&self) {
        crossterm::terminal::disable_raw_mode().expect("Failed to disable raw mode");
        io::stdout()
            .execute(cursor::Show)
            .expect("Failed running crossterm commands");
        io::stdout()
            .execute(event::DisableMouseCapture)
            .expect("Failed enabling mouse capture");
    }

    pub fn run(&mut self) {
        self.gfx.refresh_state();
        self.app.reset(&mut self.gfx);
        self.turn_on_terminal_raw_mode();

        let mut frame_start_ms;

        let (ch_writer, ch_reader) = mpsc::channel::<Event>();
        let event_thread_should_finish = Arc::new(AtomicBool::new(false));

        let event_thread = thread::spawn({
            let event_thread_should_finish = event_thread_should_finish.clone();

            move || {
                while !event_thread_should_finish.load(std::sync::atomic::Ordering::Acquire) {
                    if poll(Duration::from_millis(1)).expect("Failed polling for events") {
                        let event = read().expect("Failed reading event.");

                        trace!("Event: {:?}", event);

                        ch_writer.send(event).expect("Failed sending event.");
                    }
                }
            }
        });

        let mut events = EventGroup::new();

        while !self.should_terminate {
            frame_start_ms = get_current_ms();

            let mut new_events = vec![];
            while let Ok(e) = ch_reader.try_recv() {
                new_events.push(e);
            }
            events.events = new_events;

            for event in &events.events {
                match event {
                    Event::Key(key_event) => {
                        if key_event.code == KeyCode::Esc {
                            self.should_terminate = true;
                        }
                    }
                    Event::Resize(width, height) => {
                        self.gfx.width = *width as usize;
                        self.gfx.height = *height as usize;
                    }
                    _ => {}
                }
            }

            if !self.app.update(&events, &mut self.gfx) {
                self.should_terminate = true;
            }
            self.app.draw(&mut self.gfx);

            self.gfx.flush_buffer();

            let current_ms = get_current_ms();
            let elapsed_ms = current_ms - frame_start_ms;

            if elapsed_ms < self.target_frame_length_ms {
                std::thread::sleep(Duration::from_millis(
                    (self.target_frame_length_ms - elapsed_ms) as u64,
                ));
            }
        }

        event_thread_should_finish.store(true, std::sync::atomic::Ordering::Release);
        event_thread.join().expect("Failed joining event thread");
    }

    pub fn set_target_fps(&mut self, target_fps: u128) {
        self.target_frame_length_ms = 1_000 / target_fps;
    }

    pub fn disable_fps(&mut self) {
        self.target_frame_length_ms = 0;
    }
}

impl Drop for Terge {
    fn drop(&mut self) {
        self.turn_off_terminal_raw_mode();
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
}
