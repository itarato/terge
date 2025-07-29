use crossterm::event::{Event, KeyCode, MouseButton, MouseEventKind};
use log::{error, info};
use terge::Terge;

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

struct App {
    draw_mode_details: Option<DrawAction>,
    draw_mode_indent: DrawMode,
    current_mouse_pos: I32Point,
}

impl App {
    fn new() -> Self {
        Self {
            draw_mode_details: None,
            draw_mode_indent: DrawMode::Rect,
            current_mouse_pos: (-1, -1),
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
        self.draw_mode_details = None;
    }
}

impl terge::App for App {
    fn draw(&self, gfx: &mut terge::Gfx) {
        gfx.clear_screen();

        if let Some(draw_action) = &self.draw_mode_details {
            let start_x = draw_action.start.0.min(self.current_mouse_pos.0);
            let end_x = draw_action.start.0.max(self.current_mouse_pos.0);
            let start_y = draw_action.start.1.min(self.current_mouse_pos.1);
            let end_y = draw_action.start.1.max(self.current_mouse_pos.1);

            match draw_action.mode {
                DrawMode::Rect => {
                    for y in start_y..=end_y {
                        gfx.draw_text(BLOCK_CHAR, start_x as usize, y as usize);
                        gfx.draw_text(BLOCK_CHAR, end_x as usize, y as usize);
                    }
                    gfx.draw_text(
                        &BLOCK_CHAR.repeat((end_x - start_x) as usize),
                        start_x as usize,
                        start_y as usize,
                    );
                    gfx.draw_text(
                        &BLOCK_CHAR.repeat((end_x - start_x) as usize),
                        start_x as usize,
                        end_y as usize,
                    );
                }
                DrawMode::Line => {
                    let diff_x = end_x - start_x;
                    let diff_y = end_y - start_y;
                    if diff_x >= diff_y {
                        for x in start_x..=end_x {}
                    }
                }
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
