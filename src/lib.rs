use std::{io::Write, time::Duration};

use crossterm::event::{Event, KeyCode, poll, read};

pub trait App {
    fn reset(&mut self, gfx: &mut Gfx);
    fn update(&mut self, event: Option<Event>, gfx: &mut Gfx) -> bool;
    fn draw(&self, gfx: &mut Gfx);
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
        print!("\x1B[2J\x1B[H");
    }

    fn draw_pos(&self, x: usize, y: usize) {
        print!("\x1B[{};{}H", y + 1, x + 1);
    }

    pub fn draw_text(&self, text: &str, x: usize, y: usize) {
        self.draw_pos(x, y);
        print!("{}", text);
    }

    fn flush_buffer(&self) {
        std::io::stdout().flush().expect("Failed flushing STDOUT");
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
    }

    fn turn_off_terminal_raw_mode(&self) {
        crossterm::terminal::disable_raw_mode().expect("Failed to disable raw mode");
    }

    pub fn run(&mut self) {
        self.gfx.refresh_state();
        self.app.reset(&mut self.gfx);
        self.turn_on_terminal_raw_mode();

        let mut frame_start_ms;

        while !self.should_terminate {
            frame_start_ms = get_current_ms();

            let event = if poll(Duration::from_millis(1)).expect("Failed polling events") {
                read()
                    .map(|event| {
                        match event {
                            Event::Key(key_event) => {
                                if key_event.code == KeyCode::Esc {
                                    self.should_terminate = true;
                                }
                            }
                            Event::Resize(width, height) => {
                                self.gfx.width = width as usize;
                                self.gfx.height = height as usize;
                            }
                            _ => {}
                        }
                        event
                    })
                    .ok()
            } else {
                None
            };

            if !self.app.update(event, &mut self.gfx) {
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
    use super::*;
}
