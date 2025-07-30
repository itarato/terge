use std::collections::HashMap;

use crossterm::event::{Event, KeyCode, MouseButton, MouseEventKind};
use log::error;
use terge::{Arithmetics, I32Point, Line, Rect, Terge};

type IdType = u64;

#[derive(Debug, Clone, Copy, PartialEq)]
enum DrawIntent {
    Line,
    Rect,
}

enum Action {
    Line {
        start: I32Point,
    },
    Rect {
        start: I32Point,
    },
    DragAndDrop {
        rectangle_id: IdType,
        offset: I32Point,
    },
}

struct App {
    id_provider: u64,
    action: Option<Action>,
    draw_mode_indent: DrawIntent,
    current_mouse_pos: I32Point,
    rectangles: HashMap<IdType, Rect>,
    lines: HashMap<IdType, Line>,
}

impl App {
    fn new() -> Self {
        Self {
            id_provider: 0,
            action: None,
            draw_mode_indent: DrawIntent::Rect,
            current_mouse_pos: (-1, -1),
            rectangles: HashMap::new(),
            lines: HashMap::new(),
        }
    }

    fn get_id(&mut self) -> u64 {
        self.id_provider += 1;
        self.id_provider
    }

    fn start_draw_mode(&mut self, start: I32Point) {
        if self.action.is_some() {
            error!("Starting draw mode when there is already one.");
            return;
        }

        match self.draw_mode_indent {
            DrawIntent::Line => self.action = Some(Action::Line { start }),
            DrawIntent::Rect => self.action = Some(Action::Rect { start }),
        }
    }

    fn end_draw_mode(&mut self) {
        if let Some(action) = self.action.take() {
            match action {
                Action::Line { start } => {
                    let new_id = self.get_id();
                    self.lines.insert(
                        new_id,
                        Line {
                            start,
                            end: self.current_mouse_pos,
                        },
                    );
                }
                Action::Rect { start } => {
                    let new_id = self.get_id();
                    self.rectangles.insert(
                        new_id,
                        Rect::new_from_unordered_points(start, self.current_mouse_pos),
                    );
                }
                _ => {}
            }
        }
    }

    fn rectangle_under_cursor(&self) -> Option<(IdType, &Rect)> {
        for (id, rect) in &self.rectangles {
            if rect.point_on_header(self.current_mouse_pos) {
                return Some((*id, rect));
            }
        }

        None
    }
}

impl terge::App for App {
    fn draw(&self, gfx: &mut terge::Gfx) {
        gfx.clear_screen();

        for (_id, rect) in &self.rectangles {
            gfx.draw_rect(rect);
        }
        for (_id, line) in &self.lines {
            gfx.draw_line(line.start, line.end);
        }

        if let Some(draw_action) = &self.action {
            match draw_action {
                Action::Rect { start } => gfx.draw_rect_from_points(*start, self.current_mouse_pos),
                Action::Line { start } => gfx.draw_line(*start, self.current_mouse_pos),
                _ => {}
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
                        if let Some((rectangle_id, rect)) = self.rectangle_under_cursor() {
                            self.action = Some(Action::DragAndDrop {
                                rectangle_id,
                                offset: self.current_mouse_pos.sub(rect.start),
                            });
                        } else {
                            self.start_draw_mode((
                                mouse_event.column as i32,
                                mouse_event.row as i32,
                            ));
                        }
                    }
                    if mouse_event.kind == MouseEventKind::Up(MouseButton::Left) {
                        self.end_draw_mode();
                    }
                }
                Event::Key(key_event) => {
                    if key_event.is_press() {
                        match key_event.code {
                            KeyCode::Char('r') => self.draw_mode_indent = DrawIntent::Rect,
                            KeyCode::Char('l') => self.draw_mode_indent = DrawIntent::Line,
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }

        if let Some(Action::DragAndDrop {
            rectangle_id,
            offset,
        }) = self.action
        {
            self.rectangles
                .get_mut(&rectangle_id)
                .map(|rect| rect.start = self.current_mouse_pos.sub(offset));
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
