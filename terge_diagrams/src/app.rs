use std::collections::{HashMap, VecDeque};

use crossterm::event::KeyEvent;
use crossterm::event::{Event, KeyCode, MouseButton, MouseEvent, MouseEventKind};
use log::error;
use terge::common::{Arithmetics, U16Point, i32point_to_u16point, u16point_to_i32point};
use terge::event_group::EventGroup;
use terge::gfx::Gfx;
use terge::line::Line;
use terge::rect::Rect;

use crate::common::*;
use crate::line::*;
use crate::rect::*;
use crate::text::*;
use crate::text_editor::*;

pub struct App {
    id_provider: u64,
    action: Option<Action>,
    intent: Intent,
    current_mouse_pos: U16Point,
    current_color: usize,
    rectangles: HashMap<IdType, RectObject>,
    lines: HashMap<IdType, LineObject>,
    texts: HashMap<IdType, TextObject>,
    pointer_trace: VecDeque<PointerPoint>,
}

impl App {
    pub fn new() -> Self {
        Self {
            id_provider: 0,
            action: None,
            intent: Intent::Rect,
            current_mouse_pos: (0, 0),
            current_color: 0,
            rectangles: HashMap::new(),
            lines: HashMap::new(),
            texts: HashMap::new(),
            pointer_trace: VecDeque::new(),
        }
    }

    fn get_id(&mut self) -> u64 {
        self.id_provider += 1;
        self.id_provider
    }

    fn start_action(&mut self, start: U16Point) {
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
            Intent::Pointer => self.action = Some(Action::Pointer),
        }
    }

    fn on_mouse_left_up(&mut self) {
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
                        self.current_color,
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
                        self.current_color,
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
            Some(Action::DragText { text_id }) => {
                let rect_id = self
                    .rectangle_under_point(self.current_mouse_pos)
                    .map(|rect_obj| rect_obj.id);
                self.texts
                    .get_mut(&text_id)
                    .map(|text_obj| text_obj.anchor_rect_id = rect_id);
                self.action = None;
            }
            Some(Action::DragRectangle { .. })
            | Some(Action::ResizeRectangle { .. })
            | Some(Action::Pointer) => {
                self.action = None;
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
            self.texts.insert(
                id,
                TextObject::new(id, start, editor.lines, anchor_rect_id, self.current_color),
            );
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

    fn rectangle_header_under_point(&self, p: U16Point) -> Option<&RectObject> {
        for (_id, rect_obj) in &self.rectangles {
            if rect_obj.rect.is_point_on_header(p) {
                return Some(rect_obj);
            }
        }

        None
    }

    fn rectangle_resize_point_under_point(&self, p: U16Point) -> Option<&RectObject> {
        for (_id, rect_obj) in &self.rectangles {
            if rect_obj.rect.end() == p {
                return Some(rect_obj);
            }
        }

        None
    }

    fn rectangle_under_point(&self, p: U16Point) -> Option<&RectObject> {
        for (_id, rect_obj) in &self.rectangles {
            if rect_obj.rect.is_point_on(p) {
                return Some(rect_obj);
            }
        }

        None
    }

    fn text_edit_under_point(&self, p: U16Point) -> Option<&TextObject> {
        for (_id, text_obj) in &self.texts {
            if text_obj.is_edit_point(p) {
                return Some(text_obj);
            }
        }
        None
    }

    fn text_drag_under_point(&self, p: U16Point) -> Option<&TextObject> {
        for (_id, text_obj) in &self.texts {
            if text_obj.is_drag_point(p) {
                return Some(text_obj);
            }
        }
        None
    }

    fn line_with_start_under_point(&mut self, p: U16Point) -> Option<&mut LineObject> {
        for (_id, line_obj) in &mut self.lines {
            if line_obj.line.start == p {
                return Some(line_obj);
            }
        }
        None
    }

    fn line_with_end_under_point(&mut self, p: U16Point) -> Option<&mut LineObject> {
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

    fn on_left_mouse_button_down(&mut self, mouse_event: &MouseEvent) {
        if let Some(rect_obj) = self.rectangle_resize_point_under_point(self.current_mouse_pos) {
            self.action = Some(Action::ResizeRectangle {
                rectangle_id: rect_obj.id,
                orig_start: rect_obj.rect.start,
            });
        } else if let Some(line_obj) = self.line_with_start_under_point(self.current_mouse_pos) {
            line_obj.start_anchor_rect_id = None;
            self.action = Some(Action::DragLineStart {
                line_id: line_obj.id,
            });
        } else if let Some(line_obj) = self.line_with_end_under_point(self.current_mouse_pos) {
            line_obj.end_anchor_rect_id = None;
            self.action = Some(Action::DragLineEnd {
                line_id: line_obj.id,
            });
        } else if let Some(text_obj) = self.text_edit_under_point(self.current_mouse_pos) {
            let id = text_obj.id;

            self.action = Some(Action::Text {
                start: text_obj.start,
                editor: TextEditor::new_with_lines(text_obj.lines.clone()),
            });

            self.texts.remove(&id);
        } else if let Some(text_obj) = self.text_drag_under_point(self.current_mouse_pos) {
            self.action = Some(Action::DragText {
                text_id: text_obj.id,
            });
        } else if let Some(rect_obj) = self.rectangle_header_under_point(self.current_mouse_pos) {
            self.action = Some(Action::DragRectangle {
                rectangle_id: rect_obj.id,
                offset: u16point_to_i32point(self.current_mouse_pos)
                    .sub(u16point_to_i32point(rect_obj.rect.start)),
            });
        } else {
            self.start_action((mouse_event.column, mouse_event.row));
        }
    }

    fn current_color_code(&self) -> u8 {
        COLORS[self.current_color].0
    }

    fn delete_under_point(&mut self, p: U16Point) {
        let mut done = false;

        self.rectangles.retain(|_, rect_obj| {
            if !done && rect_obj.rect.is_point_on(p) {
                done = true;
                false
            } else {
                true
            }
        });
        // TODO cleanup dangling rect ids from lines and texts.

        if done {
            return;
        }

        self.lines.retain(|_, line_obj| {
            if !done && line_obj.line.is_point_on(p) {
                done = true;
                false
            } else {
                true
            }
        });

        if done {
            return;
        }
    }

    fn text_edit_mode_update(&mut self, key_event: &KeyEvent) {
        if let Some(Action::Text { editor, .. }) = self.action.as_mut() {
            editor.edit(&key_event);
        } else {
            unreachable!("Must be text action");
        }
    }

    fn on_key_press_not_edit_mode(&mut self, key_code: &KeyCode) {
        match *key_code {
            KeyCode::Char(c) => match c {
                'r' => self.intent = Intent::Rect,
                'l' => self.intent = Intent::Line,
                't' => self.intent = Intent::Text,
                'p' => self.intent = Intent::Pointer,
                num_c @ '0'..='9' => self.current_color = (num_c as u8 - b'0') as usize,
                _ => {}
            },
            KeyCode::Delete => self.delete_under_point(self.current_mouse_pos),
            _ => {}
        }
    }

    fn on_update_current_action(&mut self) {
        match &self.action {
            Some(Action::DragRectangle {
                rectangle_id,
                offset,
            }) => {
                self.rectangles.get_mut(&rectangle_id).map(|rect_obj| {
                    let mut new_pos = u16point_to_i32point(self.current_mouse_pos).sub(*offset);
                    if new_pos.0 < 0 {
                        new_pos.0 = 0;
                    }
                    rect_obj.rect.start = i32point_to_u16point(new_pos)
                });
            }
            Some(Action::ResizeRectangle {
                rectangle_id,
                orig_start,
            }) => {
                self.rectangles
                    .get_mut(&rectangle_id)
                    .map(|rect_obj| rect_obj.resize(*orig_start, self.current_mouse_pos));
            }
            Some(Action::DragLineStart { line_id }) => {
                self.lines
                    .get_mut(&line_id)
                    .map(|line_obj| line_obj.line.start = self.current_mouse_pos);
            }
            Some(Action::DragLineEnd { line_id }) => {
                self.lines
                    .get_mut(&line_id)
                    .map(|line_obj| line_obj.line.end = self.current_mouse_pos);
            }
            Some(Action::DragText { text_id }) => {
                self.texts
                    .get_mut(&text_id)
                    .map(|text_obj| text_obj.start = self.current_mouse_pos);
            }
            Some(Action::Pointer) => self.populate_pointer_trace(),
            Some(Action::Rect { .. })
            | Some(Action::Text { .. })
            | Some(Action::Line { .. })
            | None => {}
        }
    }

    fn on_update_lines_state(&mut self) {
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
    }

    fn on_update_text_state(&mut self) {
        for (_id, text_obj) in self.texts.iter_mut() {
            if let Some(p) = text_obj
                .anchor_rect_id
                .and_then(|id| self.rectangles.get(&id))
                .map(|rect_obj| rect_obj.rect.midpoint())
            {
                let is_text_dragged = match self.action {
                    Some(Action::DragText { text_id }) => text_id == text_obj.id,
                    _ => false,
                };

                if !is_text_dragged {
                    text_obj.start = p;
                }
            }
        }
    }

    fn on_update(&mut self) {
        self.on_update_current_action();
        self.on_update_lines_state();
        self.on_update_text_state();
        self.update_pointer_trace();
    }

    fn populate_pointer_trace(&mut self) {
        let can_have_new = if let Some(last_coord) = self.pointer_trace.front().map(|pt| pt.pos) {
            last_coord != self.current_mouse_pos
        } else {
            true
        };

        if can_have_new {
            let current_time_ms = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis();
            self.pointer_trace.push_front(PointerPoint {
                pos: self.current_mouse_pos,
                deadline: current_time_ms + 2000,
            });
        }
    }

    fn update_pointer_trace(&mut self) {
        let current_time_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();

        loop {
            if let Some(deadline) = self.pointer_trace.back().map(|pt| pt.deadline) {
                if deadline < current_time_ms {
                    self.pointer_trace.pop_back();
                    continue;
                }
            }
            break;
        }
    }
}

impl terge::App for App {
    fn draw(&self, gfx: &mut Gfx) {
        gfx.clear_screen();

        for (_id, rect_obj) in &self.rectangles {
            gfx.draw_rect(&rect_obj.rect, COLORS[rect_obj.color].0);

            if rect_obj.is_drag_point(self.current_mouse_pos)
                || rect_obj.is_resize_point(self.current_mouse_pos)
            {
                gfx.draw_text_at_point(DRAG_STR, self.current_mouse_pos, DEFAULT_COLOR_CODE);
            }
        }

        for (_id, line_obj) in &self.lines {
            gfx.draw_line(&line_obj.line, COLORS[line_obj.color].0);

            if line_obj.is_drag_point(self.current_mouse_pos) {
                gfx.draw_text_at_point(DRAG_STR, self.current_mouse_pos, DEFAULT_COLOR_CODE);
            }
        }

        for (_id, text_obj) in &self.texts {
            text_obj.draw(&gfx);

            if text_obj.is_edit_point(self.current_mouse_pos) {
                gfx.draw_text_at_point(EDIT_STR, self.current_mouse_pos, DEFAULT_COLOR_CODE);
            }
            if text_obj.is_drag_point(self.current_mouse_pos) {
                gfx.draw_text_at_point(DRAG_STR, self.current_mouse_pos, DEFAULT_COLOR_CODE);
            }
        }

        for trace in &self.pointer_trace {
            gfx.draw_text(
                POINTER_STR,
                trace.pos.0,
                trace.pos.1,
                self.current_color_code(),
            );
        }

        if let Some(draw_action) = &self.action {
            match draw_action {
                Action::Rect { start } => gfx.draw_rect_from_points(
                    *start,
                    self.current_mouse_pos,
                    self.current_color_code(),
                ),
                Action::Line { start } => gfx.draw_line_from_points(
                    *start,
                    self.current_mouse_pos,
                    self.current_color_code(),
                ),
                Action::Text { start, editor } => {
                    gfx.draw_multiline_text(
                        &editor.lines,
                        start.0,
                        start.1,
                        self.current_color_code(),
                    );
                    gfx.draw_text_to_current_pos("_");
                }
                Action::DragRectangle { .. }
                | Action::DragLineStart { .. }
                | Action::DragLineEnd { .. }
                | Action::ResizeRectangle { .. }
                | Action::DragText { .. }
                | Action::Pointer => {
                    gfx.draw_text(
                        POINTER_STR,
                        self.current_mouse_pos.0,
                        self.current_mouse_pos.1,
                        self.current_color_code(),
                    );
                }
            };
        }

        gfx.draw_text_to_current_pos("\x1B[100m");
        gfx.draw_text_uncoloured(&" ".repeat(gfx.width as usize), 0, gfx.height - 1);
        gfx.draw_text_to_current_pos("\x1B[0m");

        gfx.draw_text_uncoloured(
            &format!(
                " Intent: {:?} \x1B[100m \x1B[0m Active: {} \x1B[100m \x1B[0m Color: \x1B[{}m{}\x1B[0m ",
                self.intent,
                self.action
                    .as_ref()
                    .map(|a| a.to_string_short())
                    .unwrap_or("-"),
                COLORS[self.current_color as usize].0,
                COLORS[self.current_color as usize].1
            ),
            2,
            gfx.height - 1,
        );
    }

    fn reset(&mut self, _gfx: &mut Gfx) {}

    fn update(&mut self, events: &EventGroup, _gfx: &mut Gfx) -> bool {
        if let Some(last_mouse_pos) = events.last_mouse_pos() {
            self.current_mouse_pos = last_mouse_pos;
        }

        for e in &events.events {
            match e {
                Event::Mouse(mouse_event) => {
                    if mouse_event.kind == MouseEventKind::Down(MouseButton::Left) {
                        self.on_left_mouse_button_down(&mouse_event);
                    }
                    if mouse_event.kind == MouseEventKind::Up(MouseButton::Left) {
                        self.on_mouse_left_up();
                    }
                }
                Event::Key(key_event) => {
                    if key_event.is_press() {
                        if self.is_active_action_text() {
                            if key_event.is_enter_without_alt() {
                                self.end_text_mode();
                            } else {
                                self.text_edit_mode_update(&key_event);
                            }
                        } else {
                            self.on_key_press_not_edit_mode(&key_event.code);
                        }
                    }
                }
                _ => {}
            }
        }

        self.on_update();

        true
    }
}
