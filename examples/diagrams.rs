use crossterm::event::{Event, KeyCode, MouseButton, MouseEventKind};
use log::{error, info};
use terge::{Gfx, Terge};

type I32Point = (i32, i32);
const BLOCK_CHAR: &'static str = "█";

#[derive(Debug, Clone, Copy, PartialEq)]
enum DrawMode {
    Nothing,
    Line,
    Rect,
}

struct DrawAction {
    mode: DrawMode,
    start: I32Point,
}

fn point_pair_minmax(lhs: I32Point, rhs: I32Point) -> (i32, i32, i32, i32) {
    (
        lhs.0.min(rhs.0),
        lhs.1.min(rhs.1),
        lhs.0.max(rhs.0),
        lhs.1.max(rhs.1),
    )
}

fn draw_rect(gfx: &mut Gfx, start: I32Point, end: I32Point) {
    let (x_min, y_min, x_max, y_max) = point_pair_minmax(start, end);

    for y in y_min..=y_max {
        gfx.draw_text(BLOCK_CHAR, x_min as usize, y as usize);
        gfx.draw_text(BLOCK_CHAR, x_max as usize, y as usize);
    }
    gfx.draw_text(
        &BLOCK_CHAR.repeat((x_max - x_min) as usize),
        x_min as usize,
        y_min as usize,
    );
    gfx.draw_text(
        &BLOCK_CHAR.repeat((x_max - x_min) as usize),
        x_min as usize,
        y_max as usize,
    );
}

fn draw_line(gfx: &mut Gfx, start: I32Point, end: I32Point) {
    let (x_min, y_min, x_max, y_max) = point_pair_minmax(start, end);

    let diff_x = (end.0 - start.0) as f32;
    let diff_y = (end.1 - start.1) as f32;
    let diff_x_abs = diff_x.abs();
    let diff_y_abs = diff_y.abs();

    if diff_x_abs >= diff_y_abs {
        if diff_x != 0.0 {
            for x in x_min..=x_max {
                let y = ((diff_y / diff_x) * (x as f32 - start.0 as f32) + start.1 as f32) as usize;
                gfx.draw_text(BLOCK_CHAR, x as usize, y);
            }
        }
    } else {
        if diff_y != 0.0 {
            for y in y_min..=y_max {
                let x = ((diff_x / diff_y) * (y as f32 - start.1 as f32) + start.0 as f32) as usize;
                gfx.draw_text(BLOCK_CHAR, x, y as usize);
            }
        }
    }
}

struct Rect {
    start: I32Point,
    end: I32Point,
}

struct App {
    draw_mode_details: Option<DrawAction>,
    draw_mode_indent: DrawMode,
    current_mouse_pos: I32Point,
    rectangles: Vec<Rect>,
}

impl App {
    fn new() -> Self {
        Self {
            draw_mode_details: None,
            draw_mode_indent: DrawMode::Rect,
            current_mouse_pos: (-1, -1),
            rectangles: vec![],
        }
    }

    fn start_draw_mode(&mut self, start: I32Point) {
        if self.draw_mode_details.is_some() {
            error!("Starting draw mode when there is already one.");
            return;
        }

        if self.draw_mode_indent == DrawMode::Nothing {
            info!("Rejecting draw mode - no intent.");
            return;
        }

        self.draw_mode_details = Some(DrawAction {
            mode: self.draw_mode_indent,
            start,
        });
    }

    fn end_draw_mode(&mut self) {
        match &self.draw_mode_details {
            None => {}
            Some(action) => {}
        };

        self.draw_mode_details = None;
    }
}

impl terge::App for App {
    fn draw(&self, gfx: &mut terge::Gfx) {
        gfx.clear_screen();

        if let Some(draw_action) = &self.draw_mode_details {
            match draw_action.mode {
                DrawMode::Rect => draw_rect(gfx, draw_action.start, self.current_mouse_pos),
                DrawMode::Line => draw_line(gfx, draw_action.start, self.current_mouse_pos),
                _ => unreachable!("Draw action cannot be nothing"),
            };
        }
    }

    fn reset(&mut self, _gfx: &mut terge::Gfx) {}

    fn update(&mut self, events: &terge::EventGroup, _gfx: &mut terge::Gfx) -> bool {
        for e in &events.events {
            match e {
                Event::Mouse(mouse_event) => {
                    if mouse_event.kind == MouseEventKind::Down(MouseButton::Left) {
                        self.start_draw_mode((mouse_event.column as i32, mouse_event.row as i32));
                    }
                    if mouse_event.kind == MouseEventKind::Up(MouseButton::Left) {
                        self.end_draw_mode();
                    }
                }
                Event::Key(key_event) => {
                    if key_event.is_press() {
                        match key_event.code {
                            KeyCode::Char('r') => self.draw_mode_indent = DrawMode::Rect,
                            KeyCode::Char('l') => self.draw_mode_indent = DrawMode::Line,
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }

        if let Some(last_mouse_pos) = events.last_mouse_pos() {
            self.current_mouse_pos.0 = last_mouse_pos.0 as i32;
            self.current_mouse_pos.1 = last_mouse_pos.1 as i32;
        }

        true
    }
}

fn main() {
    pretty_env_logger::init();

    let mut engine = Terge::new(Box::new(App::new()));
    engine.set_target_fps(60);
    engine.run();
}
