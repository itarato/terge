use terge::Terge;

use crate::app::App;

mod app;
mod common;
mod line;
mod rect;
mod text;
mod text_editor;

fn main() {
    pretty_env_logger::init();

    let mut engine = Terge::new(Box::new(App::new()));
    engine.set_target_fps(120);
    engine.run();
}
