use crossterm::event::{self, Event};
use terge::{Gfx, Terge};

struct App {
    counter: u64,
    fps: u64,
    timestamp: u64,
}

fn get_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

impl App {
    fn new() -> Self {
        Self {
            counter: 0,
            fps: 0,
            timestamp: get_timestamp(),
        }
    }
}

impl terge::App for App {
    fn draw(&self, gfx: &mut Gfx) {
        gfx.clear_screen();
        gfx.draw_text(
            format!("FPS: {}", self.fps).as_str(),
            gfx.width / 2 - 4,
            gfx.height / 2,
        );
    }

    fn update(&mut self, event: Option<Event>) -> bool {
        if let Some(event) = event {
            match event {
                _ => {}
            }
        }

        let new_timestamp = get_timestamp();

        if new_timestamp > self.timestamp {
            self.fps = (self.counter + 1) / (new_timestamp - self.timestamp);
            self.counter = 0;
            self.timestamp = new_timestamp;
        } else {
            self.counter += 1;
        }

        true
    }

    fn reset(&mut self) {
        self.counter = 0;
    }
}

fn main() {
    let mut engine = Terge::new(Box::new(App::new()));
    engine.set_target_fps(24);
    engine.run();
}
