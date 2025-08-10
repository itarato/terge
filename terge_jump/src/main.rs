use terge::Terge;

mod app;
mod common;
mod player;
mod terrain;

use app::*;

fn main() {
    let mut app = Terge::new(Box::new(App::default()));
    app.set_target_fps(60);
    app.run();
}
