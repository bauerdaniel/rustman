//
// Daniel Bauer (bauerda@pm.me)
//

use bevy::prelude::*;

use super::unit::*;
use super::maze::*;
use super::ui::UI_HEIGHT;

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

fn size_scaling(
    mut query_window: Query<&mut Window>,
    mut query_scale: Query<(&UnitScale, &mut Transform)>
) {
    let window = query_window.single_mut();
    for (sprite_size, mut transform) in query_scale.iter_mut() {
        let scaling_factor = calc_scaling_factor(window.width(), window.height());
        transform.scale = Vec3::new(
            sprite_size.width * scaling_factor,
            sprite_size.height * scaling_factor,
            1.0,
        );
    }
}

fn position_translation(
    mut query_window: Query<&mut Window>,
    mut query_pos: Query<(&UnitPosition, &mut Transform)>,
) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }

    let window = query_window.single_mut();
    let scaling_factor = calc_scaling_factor(window.width(), window.height());
    let scaled_width = MAZE_WIDTH as f32 * scaling_factor; 
    let scaled_height = MAZE_HEIGHT as f32 * scaling_factor;

    for (pos, mut transform) in query_pos.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, scaled_width, MAZE_WIDTH as f32),
            convert(pos.y as f32, scaled_height, MAZE_HEIGHT as f32),
            0.0,
        );
    }
}

fn calc_scaling_factor(window_width: f32, window_height: f32) -> f32 {
    let height = window_height - UI_HEIGHT as f32;
    let scaling_factor_x = window_width / MAZE_WIDTH as f32;
    let scaling_factor_y = height / MAZE_HEIGHT as f32;
    let scaled_height = MAZE_HEIGHT as f32 * scaling_factor_x;
    if scaled_height > height { scaling_factor_y } else { scaling_factor_x }
}