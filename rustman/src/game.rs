//
// Daniel Bauer (bauerda@pm.me)
//

use bevy::prelude::*;

pub const POINTS_DOT: u32 = 10;
pub const POINTS_ENERGIZER: u32 = 50;
pub const POINTS_GHOST: u32 = 200;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Game::new())
        ;
    }
}

#[derive(Resource)]
pub struct Game {
    pub round: u32,
    pub points: u32,
    pub lifes: u32,
    pub elapsed_time_state: f32,
    pub elapsed_time_sound: f32,
    pub elapsed_time_blink: f32,
}

impl Game {
    pub fn new() -> Self {
        Self {
            round: 1,
            points: 0,
            lifes: 3,
            elapsed_time_state: 0.,
            elapsed_time_sound: 0.,
            elapsed_time_blink: 0.,
        }
    }
}
