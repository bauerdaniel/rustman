//
// Daniel Bauer (bauerda@pm.me)
//

use bevy::prelude::*;
use rand::Rng;

pub const UNIT_SIZE: u32 = 100;

#[derive(Component)]
pub struct UnitName(pub String);

//#[derive(Component)]
//pub struct UnitColor(String);

#[derive(Component)]
pub struct UnitScale {
    pub width: f32,
    pub height: f32,
}

impl UnitScale {
    pub fn square(x: f32) -> Self {
        Self { width: x, height: x }
    }
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub struct UnitPosition {
    pub x: i32,
    pub y: i32,
}

impl UnitPosition {
    pub fn move_in_direction(&mut self, direction: UnitDirection) {
        match direction {
            UnitDirection::Left => self.x -= 1,
            UnitDirection::Right => self.x += 1,
            UnitDirection::Up => self.y += 1,
            UnitDirection::Down => self.y -= 1,
            _ => {}
        };
    }

    pub fn to_vec3(&self) -> Vec3 {
        Vec3 { x: self.x as f32, y: self.y as f32, z: 0. }
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum UnitDirection {
    None,
    Left,
    Up,
    Right,
    Down,
}

impl UnitDirection {
    #[allow(dead_code)]
    pub fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::None => Self::None,
        }
    }

    pub fn random() -> Self {
        let variants = [
            UnitDirection::Left,
            UnitDirection::Up,
            UnitDirection::Right,
            UnitDirection::Down,
        ];
        let index = rand::thread_rng().gen_range(0..variants.len());
        variants[index]
    }
}
