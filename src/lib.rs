use std::{io::Write, time::Duration};

use crossterm::event::{Event, poll, read};

pub trait App {
    fn reset(&mut self);
    fn update(&mut self);
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

fn get_current_μs() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

pub struct Terge {
    app: Box<dyn App>,
    gfx: Gfx,
    target_frame_length_μs: u128,
}

impl Terge {
    pub fn new(app: Box<dyn App>) -> Self {
        Self {
            app,
            gfx: Gfx::new(),
            target_frame_length_μs: 16,
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
        self.app.reset();
        self.turn_on_terminal_raw_mode();

        let mut frame_start_μs;

        loop {
            frame_start_μs = get_current_μs();

            self.app.update();
            self.app.draw(&mut self.gfx);

            self.gfx.flush_buffer();

            if poll(Duration::from_millis(1)).expect("Failed polling events") {
                // It's guaranteed that the `read()` won't block when the `poll()`
                // function returns `true`
                match read().expect("Failed reading") {
                    // Event::FocusGained => println!("FocusGained"),
                    // Event::FocusLost => println!("FocusLost"),
                    Event::Key(event) => {
                        println!("{:?}", event);
                        break;
                    }
                    Event::Mouse(event) => println!("{:?}", event),
                    // #[cfg(feature = "bracketed-paste")]
                    // Event::Paste(data) => println!("Pasted {:?}", data),
                    Event::Resize(width, height) => {
                        self.gfx.width = width as usize;
                        self.gfx.height = height as usize;
                    }
                    _ => {}
                }
            }

            let current_μs = get_current_μs();
            let elapsed_μs = current_μs - frame_start_μs;

            if elapsed_μs < self.target_frame_length_μs {
                std::thread::sleep(Duration::from_millis(
                    (self.target_frame_length_μs - elapsed_μs) as u64,
                ));
            }
        }
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
