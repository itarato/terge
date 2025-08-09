use rand::prelude::*;
use std::collections::VecDeque;

use crossterm::event::{Event, KeyCode};
use terge::{
    Terge,
    common::{F32Point, U16Point},
    gfx::Gfx,
};

pub(crate) const PLAYER_COLOR: u8 = 95;
pub(crate) const PLAYER_VY_SLOWDOWN: f32 = 0.9;
pub(crate) const PLAYER_VY_ACC: f32 = 1.2;
pub(crate) const PLAYER_VY_MAX: f32 = 2.0;
pub(crate) const PLAYER_VY_FALLBACK_THRESHOLD: f32 = 0.05;
pub(crate) const PLAYER_JUMP_MEDIUM: f32 = -2.0;

pub(crate) const FLOOR_OFFS_FROM_BOTTOM: u16 = 6;

pub(crate) const PLAYER_SPRITE: [[&'static str; 3]; 2] =
    [[" Q", " l-", "/.\\."], [" Q", "/v", " |."]];
pub(crate) const PLAYER_SPRITE_SPEED: u64 = 20;

pub(crate) const TERRAIN_OBSTACLE_SPEED: u64 = 2;
pub(crate) const TERRAIN_OBSTACLE_COLOR: u8 = 93;

#[derive(Debug, Default)]
pub(crate) struct App {
    player: Player,
    terrain: Terrain,
}

impl App {}

impl terge::App for App {
    fn reset(&mut self, gfx: &mut terge::gfx::Gfx) {
        self.player.pos = (10.0, gfx.height as f32 - FLOOR_OFFS_FROM_BOTTOM as f32);
    }

    fn draw(&self, gfx: &mut terge::gfx::Gfx) {
        gfx.clear_screen();

        self.player.draw(gfx);
        self.terrain.draw(gfx);
    }

    fn update(
        &mut self,
        events: &terge::event_group::EventGroup,
        gfx: &mut terge::gfx::Gfx,
    ) -> bool {
        for event in &events.events {
            match &event {
                Event::Key(key_event) => match &key_event.code {
                    KeyCode::Char('w') => self.player.jump(PLAYER_JUMP_MEDIUM),
                    _ => {}
                },
                _ => {}
            }
        }

        self.player.update(gfx);
        self.terrain.update(gfx);

        true
    }
}

#[derive(Debug, Default)]
pub(crate) struct Terrain {
    obstacles: VecDeque<U16Point>,
    frame_counter: u64,
}

impl Terrain {
    pub(crate) fn update(&mut self, gfx: &mut Gfx) {
        self.frame_counter = (self.frame_counter + 1) % TERRAIN_OBSTACLE_SPEED;

        // Cleanup obstacles.
        loop {
            if let Some(obstacle) = self.obstacles.front() {
                if obstacle.0 == 0 {
                    self.obstacles
                        .pop_front()
                        .expect("Failed removing front obstacle");
                    continue;
                }
            }
            break;
        }

        // Move obstacles.
        if self.frame_counter == 0 {
            for obstacle in self.obstacles.iter_mut() {
                obstacle.0 -= 1;
            }
        }

        // New obstacles.
        let rand_u8: u8 = rand::random();
        if rand_u8 >= 250 {
            let rand_h: u8 = rand::random::<u8>() % 10 + 2;
            self.obstacles.push_back((gfx.width - 1, rand_h as u16));
        }
    }

    pub(crate) fn draw(&self, gfx: &Gfx) {
        let floor = gfx.height - FLOOR_OFFS_FROM_BOTTOM;

        for obstacle in &self.obstacles {
            for i in 0..obstacle.1 {
                gfx.draw_text("#", obstacle.0, floor - i, TERRAIN_OBSTACLE_COLOR);
            }
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct Player {
    pos: F32Point,
    v: F32Point,
    sprite_counter: u64,
}

impl Player {
    pub(crate) fn draw(&self, gfx: &Gfx) {
        let player_sprite_idx =
            self.sprite_counter / (PLAYER_SPRITE_SPEED / PLAYER_SPRITE.len() as u64);
        for i in 0..3 {
            gfx.draw_text(
                PLAYER_SPRITE[player_sprite_idx as usize][i],
                self.pos.0 as u16,
                (self.pos.1 - 2.0 + i as f32) as u16,
                PLAYER_COLOR,
            );
        }
    }

    pub(crate) fn jump(&mut self, force: f32) {
        self.v.1 = force;
    }

    pub(crate) fn update(&mut self, gfx: &mut Gfx) {
        self.update_height(gfx);
        self.sprite_counter = (self.sprite_counter + 1) % PLAYER_SPRITE_SPEED;
    }

    fn update_height(&mut self, gfx: &mut Gfx) {
        if self.v.1 < 0.0 {
            // Raising up.
            self.v.1 *= PLAYER_VY_SLOWDOWN;

            // Falling back.
            if self.v.1.abs() <= PLAYER_VY_FALLBACK_THRESHOLD {
                self.v.1 = PLAYER_VY_FALLBACK_THRESHOLD;
            }
        } else {
            // Falling down.
            self.v.1 = PLAYER_VY_MAX.min(self.v.1 * PLAYER_VY_ACC);
        }

        self.pos.1 += self.v.1;

        // Floor check.
        let floor: f32 = (gfx.height - FLOOR_OFFS_FROM_BOTTOM) as f32;
        if self.pos.1 >= floor {
            self.pos.1 = floor;
            self.v.1 = 0.0;
            return;
        }
    }
}

fn main() {
    let mut app = Terge::new(Box::new(App::default()));
    app.set_target_fps(60);
    app.run();
}

/*
  Q
  l-
 /.\.
*/
