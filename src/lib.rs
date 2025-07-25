use std::io::Write;

pub trait App {
    fn reset(&mut self);
    fn update(&mut self);
    fn draw(&self, gfx: &mut Gfx);
}

pub struct Gfx {}

impl Gfx {
    fn new() -> Self {
        Self {}
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

pub struct Terge {
    app: Box<dyn App>,
    gfx: Gfx,
}

impl Terge {
    pub fn new(app: Box<dyn App>) -> Self {
        Self {
            app,
            gfx: Gfx::new(),
        }
    }

    pub fn run(&mut self) {
        self.app.reset();

        loop {
            self.app.update();
            self.app.draw(&mut self.gfx);

            self.gfx.flush_buffer();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
