use std::collections::HashMap;

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEventKind};
use log::{debug, error};
use terge::{Arithmetics, I32Point, Line, Rect, Terge, UsizePoint, intersection_of_rect_and_line};

type IdType = u64;

fn intersection_of_rect_and_anchored_line(rect: &Rect, line: &Line) -> Option<I32Point> {
    let intersections = intersection_of_rect_and_line(rect, line);

    for p in intersections {
        if line.x_range().contains(&p.0) && line.y_range().contains(&p.1) {
            return Some(p);
        }
    }

    None
}

struct TextEditor {
    cursor: UsizePoint,
    lines: Vec<String>,
}

impl TextEditor {
    fn new() -> Self {
        Self {
            cursor: (0, 0),
            lines: vec![String::new()],
        }
    }

    fn edit(&mut self, event: &KeyEvent) {
        match event.code {
            KeyCode::Char(c) => {
                self.lines[self.cursor.1].push(c);
            }
            KeyCode::Backspace => {
                self.lines[self.cursor.1].pop();
            }
            KeyCode::Enter => {
                if event.modifiers.contains(KeyModifiers::ALT) {
                    self.cursor.1 += 1;
                    self.lines.insert(self.cursor.1, String::new());
                }
            }
            _ => {}
        }
    }
}

struct RectObject {
    id: IdType,
    rect: Rect,
}

impl RectObject {
    fn new(id: IdType, rect: Rect) -> Self {
        Self { id, rect }
    }
}

struct LineObject {
    line: Line,
    start_anchor_rect_id: Option<IdType>,
    end_anchor_rect_id: Option<IdType>,
}

impl LineObject {
    fn new_with_anchors(
        line: Line,
        start_anchor_rect_id: Option<IdType>,
        end_anchor_rect_id: Option<IdType>,
    ) -> Self {
        Self {
            line,
            start_anchor_rect_id,
            end_anchor_rect_id,
        }
    }
}

struct TextObject {
    start: I32Point,
    lines: Vec<String>,
}

impl TextObject {
    fn new(start: I32Point, lines: Vec<String>) -> Self {
        Self { start, lines }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Intent {
    Line,
    Rect,
    Text,
}

enum Action {
    Line {
        start: I32Point,
    },
    Rect {
        start: I32Point,
    },
    Text {
        start: I32Point,
        editor: TextEditor,
    },
    DragAndDrop {
        rectangle_id: IdType,
        offset: I32Point,
    },
}

impl Action {
    fn is_text(&self) -> bool {
        match self {
            Action::Text { .. } => true,
            _ => false,
        }
    }

    fn to_string_short(&self) -> &str {
        match self {
            Action::Line { .. } => "line",
            Action::Rect { .. } => "rect",
            Action::Text { .. } => "text",
            Action::DragAndDrop { .. } => "drag and drop",
        }
    }
}

struct App {
    id_provider: u64,
    action: Option<Action>,
    intent: Intent,
    current_mouse_pos: I32Point,
    rectangles: HashMap<IdType, RectObject>,
    lines: HashMap<IdType, LineObject>,
    texts: HashMap<IdType, TextObject>,
}

impl App {
    fn new() -> Self {
        Self {
            id_provider: 0,
            action: None,
            intent: Intent::Rect,
            current_mouse_pos: (-1, -1),
            rectangles: HashMap::new(),
            lines: HashMap::new(),
            texts: HashMap::new(),
        }
    }

    fn get_id(&mut self) -> u64 {
        self.id_provider += 1;
        self.id_provider
    }

    fn start_action(&mut self, start: I32Point) {
        if self.action.is_some() {
            error!("Starting draw mode when there is already one.");
            return;
        }

        match self.intent {
            Intent::Line => self.action = Some(Action::Line { start }),
            Intent::Rect => self.action = Some(Action::Rect { start }),
            Intent::Text => {
                self.action = Some(Action::Text {
                    start,
                    editor: TextEditor::new(),
                })
            }
        }
    }

    fn end_draw_mode(&mut self) {
        if let Some(Action::Line { start }) = self.action {
            let new_id = self.get_id();

            let start_anchor_rect_id = self
                .rectangle_under_point(start)
                .map(|rect_obj| rect_obj.id);
            let end_anchor_rect_id = self
                .rectangle_under_point(self.current_mouse_pos)
                .map(|rect_obj| rect_obj.id);

            self.lines.insert(
                new_id,
                LineObject::new_with_anchors(
                    Line {
                        start,
                        end: self.current_mouse_pos,
                    },
                    start_anchor_rect_id,
                    end_anchor_rect_id,
                ),
            );

            self.action = None;
        } else if let Some(Action::Rect { start }) = self.action {
            let new_id = self.get_id();
            self.rectangles.insert(
                new_id,
                RectObject::new(
                    new_id,
                    Rect::new_from_unordered_points(start, self.current_mouse_pos),
                ),
            );

            self.action = None;
        } else if let Some(Action::DragAndDrop { .. }) = self.action {
            self.action = None;
        }
    }

    fn end_text_mode(&mut self) {
        if let Some(Action::Text { start, editor }) = self.action.take() {
            let id = self.get_id();
            self.texts.insert(id, TextObject::new(start, editor.lines));
            self.action = None;
        } else {
            unreachable!("Must be text action mode")
        }
    }

    fn rectangle_header_under_point(&self, p: I32Point) -> Option<&RectObject> {
        for (_id, rect_obj) in &self.rectangles {
            if rect_obj.rect.is_point_on_header(p) {
                return Some(rect_obj);
            }
        }

        None
    }

    fn rectangle_under_point(&self, p: I32Point) -> Option<&RectObject> {
        for (_id, rect_obj) in &self.rectangles {
            if rect_obj.rect.is_point_on(p) {
                return Some(rect_obj);
            }
        }

        None
    }

    fn is_active_action_text(&self) -> bool {
        self.action
            .as_ref()
            .map(|action| action.is_text())
            .unwrap_or(false)
    }
}

impl terge::App for App {
    fn draw(&self, gfx: &mut terge::Gfx) {
        gfx.clear_screen();

        for (_id, rect_obj) in &self.rectangles {
            gfx.draw_rect(&rect_obj.rect);
        }

        for (_id, line_obj) in &self.lines {
            gfx.draw_line(&line_obj.line);
        }

        for (_id, text_obj) in &self.texts {
            gfx.draw_multiline_text(
                &text_obj.lines,
                text_obj.start.0 as usize,
                text_obj.start.1 as usize,
            );
        }

        if let Some(draw_action) = &self.action {
            match draw_action {
                Action::Rect { start } => gfx.draw_rect_from_points(*start, self.current_mouse_pos),
                Action::Line { start } => gfx.draw_line_from_points(*start, self.current_mouse_pos),
                Action::DragAndDrop { .. } => {}
                Action::Text { start, editor } => {
                    gfx.draw_multiline_text(&editor.lines, start.0 as usize, start.1 as usize);
                }
            };
        }

        gfx.draw_text(
            &format!(
                "Intent: {:?} | Active: {:?}",
                self.intent,
                self.action
                    .as_ref()
                    .map(|a| a.to_string_short())
                    .unwrap_or("-")
            ),
            0,
            gfx.height - 1,
        );
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
                        if let Some(rect_obj) =
                            self.rectangle_header_under_point(self.current_mouse_pos)
                        {
                            self.action = Some(Action::DragAndDrop {
                                rectangle_id: rect_obj.id,
                                offset: self.current_mouse_pos.sub(rect_obj.rect.start),
                            });
                        } else {
                            self.start_action((mouse_event.column as i32, mouse_event.row as i32));
                        }
                    }
                    if mouse_event.kind == MouseEventKind::Up(MouseButton::Left) {
                        self.end_draw_mode();
                    }
                }
                Event::Key(key_event) => {
                    if key_event.is_press() {
                        if self.is_active_action_text() {
                            if key_event.code == KeyCode::Enter
                                && !key_event.modifiers.contains(KeyModifiers::ALT)
                            {
                                debug!("END {:?}", key_event);
                                self.end_text_mode();
                            } else {
                                if let Some(Action::Text { editor, .. }) = self.action.as_mut() {
                                    editor.edit(&key_event);
                                } else {
                                    unreachable!("Must be text action");
                                }
                            }
                        }

                        match key_event.code {
                            KeyCode::Char(c) => match c {
                                'r' => self.intent = Intent::Rect,
                                'l' => self.intent = Intent::Line,
                                't' => self.intent = Intent::Text,
                                _ => {}
                            },
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
                .map(|rect_obj| rect_obj.rect.start = self.current_mouse_pos.sub(offset));
        }

        for (_id, line_obj) in self.lines.iter_mut() {
            if let Some(rect_obj) = line_obj
                .start_anchor_rect_id
                .and_then(|rect_id| self.rectangles.get(&rect_id))
            {
                line_obj.line.start = rect_obj.rect.midpoint();
                if let Some(intersection) =
                    intersection_of_rect_and_anchored_line(&rect_obj.rect, &line_obj.line)
                {
                    line_obj.line.start = intersection;
                }
            }

            if let Some(rect_obj) = line_obj
                .end_anchor_rect_id
                .and_then(|rect_id| self.rectangles.get(&rect_id))
            {
                line_obj.line.end = rect_obj.rect.midpoint();
                if let Some(intersection) =
                    intersection_of_rect_and_anchored_line(&rect_obj.rect, &line_obj.line)
                {
                    line_obj.line.end = intersection;
                }
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
