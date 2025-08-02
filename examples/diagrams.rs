use std::collections::HashMap;

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEventKind};
use log::{debug, error};
use terge::{
    Arithmetics, Gfx, I32Point, Line, Rect, Terge, UsizePoint, intersection_of_rect_and_line,
    point_pair_minmax,
};

type IdType = u64;

const DRAG_STR: &'static str = "+";

fn intersection_of_rect_and_anchored_line(rect: &Rect, line: &Line) -> Option<I32Point> {
    let intersections = intersection_of_rect_and_line(rect, line);

    for p in intersections {
        if line.x_range().contains(&p.0) && line.y_range().contains(&p.1) {
            return Some(p);
        }
    }

    None
}

fn draw_text_inside_rectangle(gfx: &mut Gfx, text_obj: &TextObject, rect_obj: &RectObject) {
    let start_y =
        rect_obj.rect.start.1 + (rect_obj.rect.size.1 >> 1) - (text_obj.lines.len() >> 1) as i32;
    let mid_x = rect_obj.rect.start.0 + (rect_obj.rect.size.0 >> 1);

    for (i, line) in text_obj.lines.iter().enumerate() {
        gfx.draw_text(
            &line,
            mid_x as usize - (line.len() >> 1),
            start_y as usize + i,
        );
    }
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

// TODO: Add color
// TODO: Add hover-over highlight
struct RectObject {
    id: IdType,
    rect: Rect,
}

impl RectObject {
    fn new(id: IdType, rect: Rect) -> Self {
        Self { id, rect }
    }

    fn end(&self) -> I32Point {
        self.rect.start.add(self.rect.size)
    }

    fn resize(&mut self, previous_start: I32Point, end: I32Point) {
        let (min_x, min_y, max_x, max_y) = point_pair_minmax(previous_start, end);

        self.rect.start = (min_x, min_y);
        self.rect.size = (max_x - min_x, max_y - min_y);
    }
}

struct LineObject {
    id: IdType,
    line: Line,
    start_anchor_rect_id: Option<IdType>,
    end_anchor_rect_id: Option<IdType>,
}

impl LineObject {
    fn new(id: IdType, line: Line) -> Self {
        Self {
            id,
            line,
            start_anchor_rect_id: None,
            end_anchor_rect_id: None,
        }
    }
}

struct TextObject {
    start: I32Point,
    lines: Vec<String>,
    anchor_rect_id: Option<IdType>,
}

impl TextObject {
    fn new(start: I32Point, lines: Vec<String>, anchor_rect_id: Option<IdType>) -> Self {
        Self {
            start,
            lines,
            anchor_rect_id,
        }
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
    DragRectangle {
        rectangle_id: IdType,
        offset: I32Point,
    },
    DragLineStart {
        line_id: IdType,
    },
    DragLineEnd {
        line_id: IdType,
    },
    ResizeRectangle {
        rectangle_id: IdType,
        orig_start: I32Point,
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
            Action::DragRectangle { .. } => "drag rectangle",
            Action::ResizeRectangle { .. } => "resize rectangle",
            Action::DragLineStart { .. } => "drag line start",
            Action::DragLineEnd { .. } => "drag line end",
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

    fn on_mouse_left_release(&mut self) {
        match self.action {
            Some(Action::Line { start }) => {
                let new_id = self.get_id();

                self.lines.insert(
                    new_id,
                    LineObject::new(
                        new_id,
                        Line {
                            start,
                            end: self.current_mouse_pos,
                        },
                    ),
                );

                self.update_line_start_anchor(new_id);
                self.update_line_end_anchor(new_id);

                self.action = None;
            }
            Some(Action::Rect { start }) => {
                let new_id = self.get_id();
                self.rectangles.insert(
                    new_id,
                    RectObject::new(
                        new_id,
                        Rect::new_from_unordered_points(start, self.current_mouse_pos),
                    ),
                );

                self.action = None;
            }
            Some(Action::DragLineStart { line_id }) => {
                self.update_line_start_anchor(line_id);
                self.action = None;
            }
            Some(Action::DragLineEnd { line_id }) => {
                self.update_line_end_anchor(line_id);
                self.action = None;
            }
            Some(Action::DragRectangle { .. }) | Some(Action::ResizeRectangle { .. }) => {
                self.action = None
            }
            Some(Action::Text { .. }) | None => {}
        }
    }

    fn end_text_mode(&mut self) {
        if let Some(Action::Text { start, editor }) = self.action.take() {
            let anchor_rect_id = self
                .rectangle_under_point(start)
                .map(|rect_obj| rect_obj.id);

            let id = self.get_id();
            self.texts
                .insert(id, TextObject::new(start, editor.lines, anchor_rect_id));
            self.action = None;
        } else {
            unreachable!("Must be text action mode")
        }
    }

    fn update_line_start_anchor(&mut self, line_id: IdType) {
        let anchor = self.lines.get(&line_id).and_then(|line_obj| {
            self.rectangle_under_point(line_obj.line.start)
                .map(|rect_obj| rect_obj.id)
        });

        self.lines
            .get_mut(&line_id)
            .map(|line_obj| line_obj.start_anchor_rect_id = anchor);
    }

    fn update_line_end_anchor(&mut self, line_id: IdType) {
        let anchor = self.lines.get(&line_id).and_then(|line_obj| {
            self.rectangle_under_point(line_obj.line.end)
                .map(|rect_obj| rect_obj.id)
        });

        self.lines
            .get_mut(&line_id)
            .map(|line_obj| line_obj.end_anchor_rect_id = anchor);
    }

    fn rectangle_header_under_point(&self, p: I32Point) -> Option<&RectObject> {
        for (_id, rect_obj) in &self.rectangles {
            if rect_obj.rect.is_point_on_header(p) {
                return Some(rect_obj);
            }
        }

        None
    }

    fn rectangle_resize_point_under_point(&self, p: I32Point) -> Option<&RectObject> {
        for (_id, rect_obj) in &self.rectangles {
            if rect_obj.end() == p {
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

    fn line_with_start_under_point(&mut self, p: I32Point) -> Option<&mut LineObject> {
        for (_id, line_obj) in &mut self.lines {
            if line_obj.line.start == p {
                return Some(line_obj);
            }
        }
        None
    }

    fn line_with_end_under_point(&mut self, p: I32Point) -> Option<&mut LineObject> {
        for (_id, line_obj) in &mut self.lines {
            if line_obj.line.end == p {
                return Some(line_obj);
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

            if rect_obj.end() == self.current_mouse_pos {
                gfx.draw_text_at_point(DRAG_STR, self.current_mouse_pos);
            }
        }

        for (_id, line_obj) in &self.lines {
            gfx.draw_line(&line_obj.line);

            if line_obj.line.start == self.current_mouse_pos {
                gfx.draw_text_at_point(DRAG_STR, self.current_mouse_pos);
            }
            if line_obj.line.end == self.current_mouse_pos {
                gfx.draw_text_at_point(DRAG_STR, self.current_mouse_pos);
            }
        }

        for (_id, text_obj) in &self.texts {
            if let Some(rect_obj) = text_obj
                .anchor_rect_id
                .and_then(|id| self.rectangles.get(&id))
            {
                draw_text_inside_rectangle(gfx, text_obj, rect_obj);
            } else {
                gfx.draw_multiline_text(
                    &text_obj.lines,
                    text_obj.start.0 as usize,
                    text_obj.start.1 as usize,
                );
            }
        }

        if let Some(draw_action) = &self.action {
            match draw_action {
                Action::Rect { start } => gfx.draw_rect_from_points(*start, self.current_mouse_pos),
                Action::Line { start } => gfx.draw_line_from_points(*start, self.current_mouse_pos),
                Action::DragRectangle { .. } => {}
                Action::DragLineStart { .. } => {}
                Action::DragLineEnd { .. } => {}
                Action::ResizeRectangle { .. } => {}
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
                            self.action = Some(Action::DragRectangle {
                                rectangle_id: rect_obj.id,
                                offset: self.current_mouse_pos.sub(rect_obj.rect.start),
                            });
                        } else if let Some(rect_obj) =
                            self.rectangle_resize_point_under_point(self.current_mouse_pos)
                        {
                            self.action = Some(Action::ResizeRectangle {
                                rectangle_id: rect_obj.id,
                                orig_start: rect_obj.rect.start,
                            });
                        } else if let Some(line_obj) =
                            self.line_with_start_under_point(self.current_mouse_pos)
                        {
                            line_obj.start_anchor_rect_id = None;
                            self.action = Some(Action::DragLineStart {
                                line_id: line_obj.id,
                            });
                        } else if let Some(line_obj) =
                            self.line_with_end_under_point(self.current_mouse_pos)
                        {
                            line_obj.end_anchor_rect_id = None;
                            self.action = Some(Action::DragLineEnd {
                                line_id: line_obj.id,
                            });
                        } else {
                            self.start_action((mouse_event.column as i32, mouse_event.row as i32));
                        }
                    }
                    if mouse_event.kind == MouseEventKind::Up(MouseButton::Left) {
                        self.on_mouse_left_release();
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
                        } else {
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
                }
                _ => {}
            }
        }

        // Active actions.
        if let Some(Action::DragRectangle {
            rectangle_id,
            offset,
        }) = self.action
        {
            self.rectangles
                .get_mut(&rectangle_id)
                .map(|rect_obj| rect_obj.rect.start = self.current_mouse_pos.sub(offset));
        } else if let Some(Action::ResizeRectangle {
            rectangle_id,
            orig_start,
        }) = self.action
        {
            self.rectangles
                .get_mut(&rectangle_id)
                .map(|rect_obj| rect_obj.resize(orig_start, self.current_mouse_pos));
        } else if let Some(Action::DragLineStart { line_id }) = self.action {
            self.lines
                .get_mut(&line_id)
                .map(|line_obj| line_obj.line.start = self.current_mouse_pos);
        } else if let Some(Action::DragLineEnd { line_id }) = self.action {
            self.lines
                .get_mut(&line_id)
                .map(|line_obj| line_obj.line.end = self.current_mouse_pos);
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
