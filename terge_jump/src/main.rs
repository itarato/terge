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
pub(crate) const PLAYER_VY_ACC: f32 = 1.15;
pub(crate) const PLAYER_VY_MAX: f32 = 2.0;
pub(crate) const PLAYER_VY_FALLBACK_THRESHOLD: f32 = 0.2;

//                                              Medium       Tall         Long         Short
pub(crate) const JUMP_SETTING: [F32Point; 4] = [(-2.0, 2.0), (-3.0, 2.0), (-1.0, 4.0), (-1.0, 0.5)];

pub(crate) const FLOOR_OFFS_FROM_BOTTOM: u16 = 6;

pub(crate) const PLAYER_SPRITE: [[&'static str; 3]; 2] =
    [[" Q", " l-", "/.\\."], [" Q", "/v", " |."]];
pub(crate) const PLAYER_SPRITE_SPEED: u64 = 20;

pub(crate) const TERRAIN_OBSTACLE_DEFAULT_SPEED: f32 = 1.0;
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
                    KeyCode::Char('w') => {
                        self.player.jump(JUMP_SETTING[1].0);
                        self.terrain.set_speed(JUMP_SETTING[1].1);
                    }
                    KeyCode::Char('s') => {
                        self.player.jump(JUMP_SETTING[0].0);
                        self.terrain.set_speed(JUMP_SETTING[0].1);
                    }
                    KeyCode::Char('a') => {
                        self.player.jump(JUMP_SETTING[3].0);
                        self.terrain.set_speed(JUMP_SETTING[3].1);
                    }
                    KeyCode::Char('d') => {
                        self.player.jump(JUMP_SETTING[2].0);
                        self.terrain.set_speed(JUMP_SETTING[2].1);
                    }
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
    obstacles: VecDeque<F32Point>,
    speed: f32,
}

impl Terrain {
    pub(crate) fn update(&mut self, gfx: &mut Gfx) {
        // Cleanup obstacles.
        loop {
            if let Some(obstacle) = self.obstacles.front() {
                if obstacle.0 <= 0.0 {
                    self.obstacles
                        .pop_front()
                        .expect("Failed removing front obstacle");
                    continue;
                }
            }
            break;
        }

        // Move obstacles.
        for obstacle in self.obstacles.iter_mut() {
            obstacle.0 -= self.speed;
        }

        // New obstacles.
        let rand_u8: u8 = rand::random();
        if rand_u8 >= 250 {
            let rand_h: u8 = rand::random::<u8>() % 10 + 2;
            self.obstacles
                .push_back(((gfx.width - 1) as f32, rand_h as f32));
        }

        // Regulate speed.
        let diff = self.speed - TERRAIN_OBSTACLE_DEFAULT_SPEED;
        if diff != 0.0 {
            static SPEED_ADJUST: f32 = 0.95;
            let new_diff = diff * SPEED_ADJUST;
            self.speed = TERRAIN_OBSTACLE_DEFAULT_SPEED + new_diff;

            if (self.speed - TERRAIN_OBSTACLE_DEFAULT_SPEED).abs() < 0.1 {
                self.speed = TERRAIN_OBSTACLE_DEFAULT_SPEED;
            }
        }
    }

    pub(crate) fn draw(&self, gfx: &Gfx) {
        let floor = gfx.height - FLOOR_OFFS_FROM_BOTTOM;

        for obstacle in &self.obstacles {
            for i in 0..obstacle.1 as u16 {
                gfx.draw_text("#", obstacle.0 as u16, floor - i, TERRAIN_OBSTACLE_COLOR);
            }
        }
    }

    pub(crate) fn set_speed(&mut self, speed: f32) {
        self.speed = speed;
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
