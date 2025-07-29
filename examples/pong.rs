use terge::{EventGroup, Gfx, Terge};

#[derive(Debug, Default)]
struct Coord {
    x: i32,
    y: i32,
}

#[derive(Debug, Default)]
struct App {
    ball_pos: Coord,
    ball_v: Coord,
    pad_x: i32,
}

impl App {
    fn new() -> Self {
        Self::default()
    }
}

impl terge::App for App {
    fn draw(&self, gfx: &mut Gfx) {
        gfx.clear_screen();

        gfx.draw_text("████████", self.pad_x as usize, gfx.height - 1);
        gfx.draw_text("O", self.ball_pos.x as usize, self.ball_pos.y as usize);
    }

    fn update(&mut self, events: &EventGroup, gfx: &mut Gfx) -> bool {
        if let Some((x, _y)) = events.last_mouse_pos() {
            self.pad_x = x as i32;
        }

        let ball_next_x = self.ball_pos.x + self.ball_v.x;
        let ball_next_y = self.ball_pos.y + self.ball_v.y;

        if ball_next_x <= 0 || ball_next_x >= gfx.width as i32 {
            self.ball_v.x *= -1;
        }
        if ball_next_y <= 0 || ball_next_y >= gfx.height as i32 - 1 {
            self.ball_v.y *= -1;
        }
        self.ball_pos.x += self.ball_v.x;
        self.ball_pos.y += self.ball_v.y;

        true
    }

    fn reset(&mut self, gfx: &mut Gfx) {
        self.pad_x = (gfx.width / 2) as i32;
        self.ball_pos.x = (gfx.width / 2) as i32;
        self.ball_pos.y = (gfx.height / 2) as i32;

        self.ball_v.x = 1;
        self.ball_v.y = 1;
    }
}

fn main() {
    pretty_env_logger::init();

    let mut engine = Terge::new(Box::new(App::new()));
    engine.set_target_fps(60);
    engine.run();
}
