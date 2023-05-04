//
// Daniel Bauer (bauerda@pm.me)
//

use bevy::prelude::*;

#[derive(Component)]
pub struct UnitSize {
    pub width: f32,
    pub height: f32,
}

impl UnitSize {
    pub fn square(x: f32) -> Self {
        Self { width: x, height: x }
    }
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub struct UnitPosition {
    pub x: i32,
    pub y: i32,
}

#[derive(PartialEq, Copy, Clone)]
pub enum UnitDirection {
    None,
    Left,
    Up,
    Right,
    Down,
}

/*
impl UnitDirection {
    pub fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}
*/
