use std::{
    io::{self},
    sync::{Arc, atomic::AtomicBool, mpsc},
    thread,
    time::Duration,
};

use crossterm::{
    ExecutableCommand, cursor,
    event::{self, Event, KeyCode, poll, read},
};
use log::trace;

pub mod common;
pub mod event_group;
pub mod gfx;
pub mod line;
pub mod rect;

use event_group::*;
use gfx::*;
use rect::*;

pub trait App {
    fn reset(&mut self, gfx: &mut Gfx);
    fn update(&mut self, events: &EventGroup, gfx: &mut Gfx) -> bool;
    fn draw(&self, gfx: &mut Gfx);
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
