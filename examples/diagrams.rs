use crossterm::event::{Event, KeyCode, MouseButton, MouseEventKind};
use log::{error, info};
use terge::{I32Point, Terge};

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

struct Rect {
    start: I32Point,
    end: I32Point,
}

struct Line {
    start: I32Point,
    end: I32Point,
}

struct App {
    draw_mode_details: Option<DrawAction>,
    draw_mode_indent: DrawMode,
    current_mouse_pos: I32Point,
    rectangles: Vec<Rect>,
    lines: Vec<Line>,
}

impl App {
    fn new() -> Self {
        Self {
            draw_mode_details: None,
            draw_mode_indent: DrawMode::Rect,
            current_mouse_pos: (-1, -1),
            rectangles: vec![],
            lines: vec![],
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
            Some(action) => match action.mode {
                DrawMode::Line => {
                    self.lines.push(Line {
                        start: action.start,
                        end: self.current_mouse_pos,
                    });
                }
                DrawMode::Rect => {
                    self.rectangles.push(Rect {
                        start: action.start,
                        end: self.current_mouse_pos,
                    });
                }
                _ => {}
            },
        };

        self.draw_mode_details = None;
    }
}

impl terge::App for App {
    fn draw(&self, gfx: &mut terge::Gfx) {
        gfx.clear_screen();

        for rect in &self.rectangles {
            gfx.draw_rect(rect.start, rect.end);
        }
        for line in &self.lines {
            gfx.draw_line(line.start, line.end);
        }

        if let Some(draw_action) = &self.draw_mode_details {
            match draw_action.mode {
                DrawMode::Rect => gfx.draw_rect(draw_action.start, self.current_mouse_pos),
                DrawMode::Line => gfx.draw_line(draw_action.start, self.current_mouse_pos),
                _ => unreachable!("Draw action cannot be nothing"),
            };
        }
    }

    fn reset(&mut self, _gfx: &mut terge::Gfx) {}

    fn update(&mut self, events: &terge::EventGroup, _gfx: &mut terge::Gfx) -> bool {
        if let Some(last_mouse_pos) = events.last_mouse_pos() {
            self.current_mouse_pos.0 = last_mouse_pos.0 as i32;
            self.current_mouse_pos.1 = last_mouse_pos.1 as i32;
        }

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

        true
    }
}

fn main() {
    pretty_env_logger::init();

    let mut engine = Terge::new(Box::new(App::new()));
    engine.set_target_fps(60);
    engine.run();
}
