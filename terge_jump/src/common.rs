use terge::{common::F32Point, gfx::Gfx};

pub(crate) const PLAYER_COLOR: u8 = 97;
pub(crate) const PLAYER_VY_SLOWDOWN: f32 = 0.9;
pub(crate) const PLAYER_VY_ACC: f32 = 1.15;
pub(crate) const PLAYER_VY_MAX: f32 = 2.0;
pub(crate) const PLAYER_VY_FALLBACK_THRESHOLD: f32 = 0.2;
pub(crate) const PLAYER_X: u16 = 10;

//                                              Medium       Tall         Long         Short
pub(crate) const JUMP_SETTING: [F32Point; 4] = [(-1.7, 1.1), (-2.2, 1.2), (-1.0, 2.5), (-1.0, 0.4)];

pub(crate) const FLOOR_OFFS_FROM_BOTTOM: u16 = 6;

pub(crate) const PLAYER_SPRITE: [[&'static str; 2]; 4] =
    [["O", "^"], ["O", "<"], ["O", "v"], ["O", ">"]];
pub(crate) const PLAYER_SPRITE_SPEED: u64 = 20;

pub(crate) const TERRAIN_OBSTACLE_DEFAULT_SPEED: f32 = 1.0;
pub(crate) const TERRAIN_OBSTACLE_COLORS: [u8; 2] = [91, 97];

pub(crate) fn floor(gfx: &Gfx) -> u16 {
    gfx.height - FLOOR_OFFS_FROM_BOTTOM
}

#[derive(Debug)]
pub(crate) enum DecorationType {
    Stone,
    GrassSmall,
    GrassMedium,
    GrassLeanLeft,
    GrassLeanRight,
}

impl DecorationType {
    pub(crate) fn random() -> Self {
        match rand::random::<u8>() % 5 {
            0 => Self::Stone,
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
    pub(crate) ty: DecorationType,
    pub(crate) x: f32,
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
    pub(crate) fn random() -> Self {
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
