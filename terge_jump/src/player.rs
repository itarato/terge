use terge::{
    common::{F32Point, Gravity, U16Point},
    gfx::Gfx,
};

use crate::common::*;

#[derive(Debug)]
pub(crate) struct Player {
    pub(crate) pos: F32Point,
    pub(crate) v: F32Point,
    pub(crate) sprite_counter: u64,
    pub(crate) dead: bool,
    pub(crate) bloods: Vec<(F32Point, F32Point)>,
    blood_g: Gravity,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            blood_g: Gravity::new(1.04, 0.2),
            pos: Default::default(),
            v: Default::default(),
            sprite_counter: 0,
            dead: false,
            bloods: vec![],
        }
    }
}

impl Player {
    pub(crate) fn reset(&mut self) {
        self.dead = false;
    }

    pub(crate) fn draw(&self, gfx: &Gfx) {
        let player_sprite_idx =
            self.sprite_counter / (PLAYER_SPRITE_SPEED / PLAYER_SPRITE.len() as u64);
        let sprite = PLAYER_SPRITE[player_sprite_idx as usize];
        for i in 0..sprite.len() {
            gfx.draw_text(
                sprite[i],
                self.pos.0 as u16,
                (self.pos.1 - sprite.len() as f32 + 1.0 + i as f32) as u16,
                PLAYER_COLOR,
            );
        }

        if self.dead {
            gfx.draw_text("▁▁▁▁▁▁▂▂▂▃", 0, floor(gfx), 91);

            for (blood_pos, _blood_v) in &self.bloods {
                gfx.draw_text("*", blood_pos.0 as u16, blood_pos.1 as u16, 31);
            }
        }
    }

    fn is_on_ground(&self, gfx: &Gfx) -> bool {
        let floor: f32 = floor(gfx) as f32;
        (floor - self.pos.1).abs() < 0.1
    }

    pub(crate) fn jump(&mut self, force: f32, gfx: &Gfx) -> bool {
        if !self.dead && self.is_on_ground(gfx) {
            self.v.1 = force;
            true
        } else {
            false
        }
    }

    pub(crate) fn update(&mut self, gfx: &mut Gfx) {
        self.update_height(gfx);
        self.update_blood(gfx);
        self.sprite_counter = (self.sprite_counter + 1) % PLAYER_SPRITE_SPEED;
    }

    fn update_blood(&mut self, gfx: &Gfx) {
        for (blood_pos, blood_v) in &mut self.bloods {
            blood_v.1 += 0.01;
            blood_pos.0 += blood_v.0;
            blood_pos.1 += blood_v.1;

            self.blood_g.apply(blood_pos, blood_v);
        }

        self.bloods.retain(|(blood_pos, _blood_v)| {
            blood_pos.0 >= 0.0
                && blood_pos.1 >= 0.0
                && blood_pos.0 < gfx.width as f32
                && blood_pos.1 < gfx.height as f32
        });
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

    pub(crate) fn frame(&self) -> (U16Point, U16Point) {
        (
            (self.pos.0 as u16, (self.pos.1) as u16 - 1),
            ((self.pos.0) as u16, self.pos.1 as u16),
        )
    }

    pub(crate) fn die(&mut self) {
        if self.dead {
            return;
        }

        self.dead = true;

        for _ in 0..32 {
            let blood_pos = self.pos;
            let blood_v = (
                rand::random::<f32>() % 1.0 - 0.5,
                rand::random::<f32>() % 1.0 - 0.5,
            );
            self.bloods.push((blood_pos, blood_v));
        }
    }
}
