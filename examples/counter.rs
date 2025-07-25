use terge::{Gfx, Terge};

struct App {
    counter: usize,
}

impl App {
    fn new() -> Self {
        Self { counter: 0 }
    }
}

impl terge::App for App {
    fn draw(&self, gfx: &mut Gfx) {
        gfx.clear_screen();
        gfx.draw_text(format!("Hello World: {}", self.counter).as_str(), 20, 10);
    }

    fn update(&mut self) {
        self.counter += 1;
    }

    fn reset(&mut self) {
        self.counter = 0;
    }
}

fn main() {
    let mut engine = Terge::new(Box::new(App::new()));
    engine.run();
}
