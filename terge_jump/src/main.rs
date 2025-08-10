use std::collections::VecDeque;

use crossterm::event::{Event, KeyCode};
use rand::random;
use terge::{
    Terge,
    common::{F32Point, U16Point, u16_range_overlap, u16_value_included_in_range},
    gfx::Gfx,
};

pub(crate) const PLAYER_COLOR: u8 = 97;
pub(crate) const PLAYER_VY_SLOWDOWN: f32 = 0.9;
pub(crate) const PLAYER_VY_ACC: f32 = 1.15;
pub(crate) const PLAYER_VY_MAX: f32 = 2.0;
pub(crate) const PLAYER_VY_FALLBACK_THRESHOLD: f32 = 0.2;
pub(crate) const PLAYER_X: u16 = 10;

//                                              Medium       Tall         Long         Short
pub(crate) const JUMP_SETTING: [F32Point; 4] = [(-1.7, 1.1), (-2.2, 1.2), (-1.0, 2.5), (-1.0, 0.4)];

pub(crate) const FLOOR_OFFS_FROM_BOTTOM: u16 = 6;

pub(crate) const PLAYER_SPRITE: [[&'static str; 1]; 4] = [["🯅"], ["🯇"], ["🯅"], ["🯈"]];
pub(crate) const PLAYER_SPRITE_SPEED: u64 = 20;

pub(crate) const TERRAIN_OBSTACLE_DEFAULT_SPEED: f32 = 1.0;
pub(crate) const TERRAIN_OBSTACLE_COLORS: [u8; 2] = [91, 97];

fn floor(gfx: &Gfx) -> u16 {
    gfx.height - FLOOR_OFFS_FROM_BOTTOM
}

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

#[derive(Debug)]
pub(crate) enum DecorationType {
    GrassMini,
    GrassSmall,
    GrassMedium,
    GrassLeanLeft,
    GrassLeanRight,
}

impl DecorationType {
    fn random() -> Self {
        match rand::random::<u8>() % 5 {
            0 => Self::GrassMini,
            1 => Self::GrassSmall,
            2 => Self::GrassMedium,
            3 => Self::GrassLeanLeft,
            4 => Self::GrassLeanRight,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct Decoration {
    ty: DecorationType,
    x: f32,
}

impl Decoration {
    pub(crate) fn new(ty: DecorationType, x: f32) -> Self {
        Self { ty, x }
    }
}

pub(crate) enum ObstacleType {
    OneSmall,
    LongSmall,
    OneMedium,
    TwoMedium,
    ThreeMedium,
    OneTall,
    TwoTall,
}

impl ObstacleType {
    fn random() -> Self {
        match rand::random::<u8>() % 7 {
            0 => Self::OneSmall,
            1 => Self::LongSmall,
            2 => Self::OneMedium,
            3 => Self::TwoMedium,
            4 => Self::ThreeMedium,
            5 => Self::OneTall,
            6 => Self::TwoTall,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct Terrain {
    obstacles: VecDeque<(f32, U16Point)>,
    speed: f32,
    decorations: VecDeque<Decoration>,
    game_over: bool,
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
            let decor_str = match decor.ty {
                DecorationType::GrassMini => ".",
                DecorationType::GrassSmall => ",",
                DecorationType::GrassMedium => "v",
                DecorationType::GrassLeanLeft => "╮",
                DecorationType::GrassLeanRight => "╭",
            };
            gfx.draw_text(decor_str, decor.x as u16, floor, 92);
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

#[derive(Debug, Default)]
pub(crate) struct Player {
    pos: F32Point,
    v: F32Point,
    sprite_counter: u64,
    pub dead: bool,
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

    pub(crate) fn frame(&self) -> (U16Point, U16Point) {
        (
            (self.pos.0 as u16, (self.pos.1) as u16),
            ((self.pos.0) as u16, self.pos.1 as u16),
        )
    }

    pub(crate) fn die(&mut self) {
        self.dead = true;
    }
}

fn main() {
    let mut app = Terge::new(Box::new(App::default()));
    app.set_target_fps(60);
    app.run();
}
