use std::collections::VecDeque;

use terge::{
    common::{U16Point, u16_range_overlap, u16_value_included_in_range},
    gfx::Gfx,
};

use crate::common::*;

#[derive(Debug, Default)]
pub(crate) struct Terrain {
    obstacles: VecDeque<(f32, U16Point)>,
    speed: f32,
    decorations: VecDeque<Decoration>,
    pub(crate) game_over: bool,
    obstacle_delay: u16,
}

impl Terrain {
    pub(crate) fn reset(&mut self) {
        self.obstacles.clear();
        self.decorations.clear();
        self.speed = TERRAIN_OBSTACLE_DEFAULT_SPEED;
        self.game_over = false;
    }

    pub(crate) fn update(&mut self, gfx: &mut Gfx) {
        // Move obstacles.
        for obstacle in &mut self.obstacles {
            obstacle.0 -= self.speed;
        }

        // Move decorations.
        for decor in &mut self.decorations {
            decor.x -= self.speed;
        }

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

        // Cleanup decorations.
        loop {
            if let Some(decor) = self.decorations.front() {
                if decor.x <= 0.0 {
                    self.decorations
                        .pop_front()
                        .expect("Failed removing front decoration");
                    continue;
                }
            }
            break;
        }

        // New obstacles.
        let last_obstacle_enough_far = self
            .obstacles
            .back()
            .map(|obstacle| gfx.width as f32 - obstacle.0 > self.obstacle_delay as f32)
            .unwrap_or(true);

        if last_obstacle_enough_far {
            let rand_u8: u8 = rand::random();
            let floor = floor(gfx);
            if rand_u8 >= 220 || true {
                match ObstacleType::random() {
                    ObstacleType::OneSmall => {
                        self.obstacles
                            .push_back((gfx.width as f32, (floor - 2, floor)));
                        self.obstacle_delay = 30;
                    }
                    ObstacleType::OneTall => {
                        self.obstacles
                            .push_back((gfx.width as f32, (floor - 16, floor)));
                        self.obstacle_delay = 70;
                    }
                    ObstacleType::TwoTall => {
                        self.obstacles
                            .push_back((gfx.width as f32, (floor - 16, floor)));
                        self.obstacles
                            .push_back((gfx.width as f32 + 8.0, (floor - 16, floor)));
                        self.obstacle_delay = 70;
                    }
                    ObstacleType::LongSmall => {
                        for i in -4i32..=4i32 {
                            self.obstacles.push_back((
                                gfx.width as f32 + (i as f32 + 4.0) * 5.0,
                                (floor - 4 + i.abs() as u16 / 2, floor),
                            ));
                        }
                        self.obstacle_delay = 70;
                    }
                    ObstacleType::OneMedium => {
                        self.obstacles
                            .push_back((gfx.width as f32, (floor - 10, floor)));
                        self.obstacle_delay = 50;
                    }
                    ObstacleType::TwoMedium => {
                        self.obstacles
                            .push_back((gfx.width as f32, (floor - 10, floor)));
                        self.obstacles
                            .push_back((gfx.width as f32 + 8.0, (floor - 10, floor)));
                        self.obstacle_delay = 55;
                    }
                    ObstacleType::ThreeMedium => {
                        self.obstacles
                            .push_back((gfx.width as f32, (floor - 10, floor)));
                        self.obstacles
                            .push_back((gfx.width as f32 + 6.0, (floor - 10, floor)));
                        self.obstacles
                            .push_back((gfx.width as f32 + 12.0, (floor - 10, floor)));
                        self.obstacle_delay = 60;
                    }
                }
            }
        }

        // New decoration.
        let last_decoration_enough_far = self
            .decorations
            .back()
            .map(|decor| gfx.width as f32 - decor.x as f32 > 2.0)
            .unwrap_or(true);
        if last_decoration_enough_far {
            let rand_u8: u8 = rand::random();
            if rand_u8 >= 200 {
                self.decorations.push_back(Decoration::new(
                    DecorationType::random(),
                    (gfx.width - 1) as f32,
                ));
            }
        }

        // Regulate speed.
        if self.game_over {
            self.speed *= 0.95;
        } else {
            let diff = self.speed - TERRAIN_OBSTACLE_DEFAULT_SPEED;
            if diff != 0.0 {
                static SPEED_ADJUST: f32 = 0.98;
                let new_diff = diff * SPEED_ADJUST;
                self.speed = TERRAIN_OBSTACLE_DEFAULT_SPEED + new_diff;

                if (self.speed - TERRAIN_OBSTACLE_DEFAULT_SPEED).abs() < 0.1 {
                    self.speed = TERRAIN_OBSTACLE_DEFAULT_SPEED;
                }
            }
        }
    }

    pub(crate) fn draw(&self, gfx: &Gfx) {
        let floor = floor(gfx);

        for decor in &self.decorations {
            let (decor_str, color) = match decor.ty {
                DecorationType::Stone => ("🬞", 90),
                DecorationType::GrassSmall => ("🬞", 32),
                DecorationType::GrassMedium => ("🬵", 32),
                DecorationType::GrassLeanLeft => ("╮", 92),
                DecorationType::GrassLeanRight => ("╭", 92),
            };
            gfx.draw_text(decor_str, decor.x as u16, floor, color);
        }

        for (obstacle_x, obstacle_y) in &self.obstacles {
            if (*obstacle_x as u16) < gfx.width {
                let obstacle_height = obstacle_y.1 - obstacle_y.0;
                for i in 0..obstacle_height as u16 {
                    gfx.draw_text(
                        "▓",
                        *obstacle_x as u16,
                        floor - i,
                        TERRAIN_OBSTACLE_COLORS[i as usize % TERRAIN_OBSTACLE_COLORS.len()],
                    );
                }
            }
        }

        gfx.draw_text(&"▒".repeat(gfx.width as usize), 0, floor + 1, 32);
        gfx.draw_text(&"▓".repeat(gfx.width as usize), 0, floor + 2, 33);
        gfx.draw_text(&"░".repeat(gfx.width as usize), 0, floor + 3, 33);
    }

    pub(crate) fn set_speed(&mut self, speed: f32) {
        self.speed = speed;
    }

    pub(crate) fn did_collide_with_frame(&self, frame: (U16Point, U16Point)) -> bool {
        for (obstacle_x, obstacle_y) in &self.obstacles {
            if u16_value_included_in_range(*obstacle_x as u16, (frame.0.0, frame.1.0)) {
                if u16_range_overlap((frame.0.1, frame.1.1), *obstacle_y) {
                    return true;
                }
            }
        }

        false
    }
}
