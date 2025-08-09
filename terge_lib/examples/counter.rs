use terge::{Terge, event_group::EventGroup, gfx::Gfx};

struct App {
    counter: u64,
    fps: u64,
    timestamp: u64,
    ch: Option<char>,
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
            ch: None,
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
            94,
        );
    }

    fn update(&mut self, events: &EventGroup, _gfx: &mut Gfx) -> bool {
        let new_timestamp = get_timestamp();

        if new_timestamp > self.timestamp {
            self.fps = (self.counter + 1) / (new_timestamp - self.timestamp);
            self.counter = 0;
            self.timestamp = new_timestamp;
        } else {
            self.counter += 1;
        }

        self.ch = events.first_pressed_char();

        true
    }

    fn reset(&mut self, _gfx: &mut Gfx) {
        self.counter = 0;
    }
}

fn main() {
    pretty_env_logger::init();

    let mut engine = Terge::new(Box::new(App::new()));
    engine.set_target_fps(120);
    // engine.disable_fps();
    engine.run();
}
