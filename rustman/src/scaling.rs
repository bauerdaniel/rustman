//
// Daniel Bauer (bauerda@pm.me)
//

use bevy::prelude::*;

use super::unit::*;
use super::maze::*;

pub const GAME_WIDTH: u32 = MAZE_WIDTH;
pub const GAME_HEIGHT: u32 = MAZE_HEIGHT;

pub struct ScalingPlugin;

impl Plugin for ScalingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems((
                position_translation,
                size_scaling,
            ))
        ;
    }
}

fn size_scaling(mut windows: Query<&mut Window>, mut q: Query<(&UnitSize, &mut Transform)>) {
    let window = windows.single_mut();
    for (sprite_size, mut transform) in q.iter_mut() {
        
        let height = window.height() as f32 - 100.;

        let scaling_factor_x = window.width() as f32 / GAME_WIDTH as f32;
        let scaling_factor_y = height as f32 / GAME_HEIGHT as f32;
        let mut scaling_factor = scaling_factor_x;

        if GAME_HEIGHT as f32 * scaling_factor > height {
            scaling_factor = scaling_factor_y;
        }

        //let new_width = sprite_size.width / GAME_WIDTH as f32 * window.width() as f32;
        //let new_height = sprite_size.height / GAME_HEIGHT as f32 * window.height() as f32;

        transform.scale = Vec3::new(
            sprite_size.width * scaling_factor,
            sprite_size.height * scaling_factor,
            1.0,
        );
    }
}

fn position_translation(mut windows: Query<&mut Window>, mut q: Query<(&UnitPosition, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }
    let window = windows.single_mut();

    let height = window.height() as f32 - 100.;

    let scaling_factor_x = window.width() as f32 / GAME_WIDTH as f32;
    let scaling_factor_y = height / GAME_HEIGHT as f32;
    let mut scaling_factor = scaling_factor_x;

    if GAME_HEIGHT as f32 * scaling_factor > height {
        scaling_factor = scaling_factor_y;
    }
    let scaled_width = GAME_WIDTH as f32 * scaling_factor; 
    let scaled_height = GAME_HEIGHT as f32 * scaling_factor; 
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, scaled_width as f32, GAME_WIDTH as f32),
            convert(pos.y as f32, scaled_height, GAME_HEIGHT as f32),
            0.0,
        );
    }
}
