use crossterm::event::Event;
use crossterm::event::KeyCode;

use crate::common::*;
use crate::player::*;
use crate::terrain::*;

#[derive(Debug, Default)]
pub(crate) struct App {
    player: Player,
    terrain: Terrain,
}

impl App {}

impl terge::App for App {
    fn reset(&mut self, gfx: &mut terge::gfx::Gfx) {
        self.player.pos = (PLAYER_X as f32, floor(gfx) as f32);

        self.player.reset();
        self.terrain.reset();
    }

    fn draw(&self, gfx: &mut terge::gfx::Gfx) {
        gfx.clear_screen();

        self.terrain.draw(gfx);
        self.player.draw(gfx);

        if self.player.dead {
            gfx.draw_text("-== DEAD ==-", gfx.width / 2 - 6, gfx.height / 3, 91);
        }
    }

    fn update(
        &mut self,
        events: &terge::event_group::EventGroup,
        gfx: &mut terge::gfx::Gfx,
    ) -> bool {
        for event in &events.events {
            match &event {
                Event::Key(key_event) => match &key_event.code {
                    KeyCode::Char('w') => {
                        if self.player.jump(JUMP_SETTING[1].0, &gfx) {
                            self.terrain.set_speed(JUMP_SETTING[1].1);
                        }
                    }
                    KeyCode::Char('s') => {
                        if self.player.jump(JUMP_SETTING[0].0, gfx) {
                            self.terrain.set_speed(JUMP_SETTING[0].1);
                        }
                    }
                    KeyCode::Char('a') => {
                        if self.player.jump(JUMP_SETTING[3].0, gfx) {
                            self.terrain.set_speed(JUMP_SETTING[3].1);
                        }
                    }
                    KeyCode::Char('d') => {
                        if self.player.jump(JUMP_SETTING[2].0, gfx) {
                            self.terrain.set_speed(JUMP_SETTING[2].1);
                        }
                    }
                    KeyCode::Char('r') => {
                        self.reset(gfx);
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        self.player.update(gfx);
        self.terrain.update(gfx);

        if self.terrain.did_collide_with_frame(self.player.frame()) {
            self.player.die();
            self.terrain.game_over = true;
        }

        true
    }
}
