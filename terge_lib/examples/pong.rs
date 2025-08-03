use terge::{
    Terge,
    common::{I32Point, U16Point},
    event_group::EventGroup,
    gfx::Gfx,
};

#[derive(Debug, Default)]
struct App {
    ball_pos: U16Point,
    ball_v: I32Point,
    pad_x: u16,
}

impl App {
    fn new() -> Self {
        Self::default()
    }
}

impl terge::App for App {
    fn draw(&self, gfx: &mut Gfx) {
        gfx.clear_screen();

        gfx.draw_text_uncoloured("████████", self.pad_x, gfx.height - 1);
        gfx.draw_text_uncoloured("O", self.ball_pos.0, self.ball_pos.1);
    }

    fn update(&mut self, events: &EventGroup, gfx: &mut Gfx) -> bool {
        if let Some((x, _y)) = events.last_mouse_pos() {
            self.pad_x = x;
        }

        let ball_next_x = self.ball_pos.0 as i32 + self.ball_v.0;
        let ball_next_y = self.ball_pos.1 as i32 + self.ball_v.1;

        if ball_next_x <= 0 || ball_next_x >= gfx.width as i32 {
            self.ball_v.0 *= -1;
        }
        if ball_next_y <= 0 || ball_next_y >= gfx.height as i32 - 1 {
            self.ball_v.1 *= -1;
        }
        self.ball_pos.0 = (self.ball_pos.0 as i32 + self.ball_v.0) as u16;
        self.ball_pos.1 = (self.ball_pos.1 as i32 + self.ball_v.1) as u16;

        true
    }

    fn reset(&mut self, gfx: &mut Gfx) {
        self.pad_x = gfx.width / 2;
        self.ball_pos.0 = gfx.width / 2;
        self.ball_pos.1 = gfx.height / 2;

        self.ball_v.0 = 1;
        self.ball_v.1 = 1;
    }
}

fn main() {
    pretty_env_logger::init();

    let mut engine = Terge::new(Box::new(App::new()));
    engine.set_target_fps(60);
    engine.run();
}
