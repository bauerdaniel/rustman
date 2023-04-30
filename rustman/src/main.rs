//
// Daniel Bauer (bauerda@pm.me)
//

// https://docs.rs/bevy/latest/bevy/index.html
// https://mbuffett.com/posts/bevy-snake-tutorial/

mod maze;
mod unit;
mod pacman;

use bevy::{
    prelude::*,
    window::PresentMode,
    render::color::Color
};

use maze::*;
use unit::*;
use pacman::*;

const FIXED_TIMESTEP: f32 = 0.004;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Rustman".into(),
                    resolution: (1280., 426.).into(),
                    present_mode: PresentMode::AutoVsync,
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()})
            .set(ImagePlugin::default_nearest()))
        // Setup
        .add_startup_systems((
            setup_camera,
            setup_maze,
            spawn_pacman,
            spawn_dots
        ))
        // Scaling
        .add_systems((
            position_translation,
            size_scaling
        ))
        // Player Movement
        .add_system(pacman_movement_input.before(pacman_movement))
        .add_system(pacman_movement.in_schedule(CoreSchedule::FixedUpdate))
        .insert_resource(FixedTime::new_from_secs(FIXED_TIMESTEP))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn size_scaling(mut windows: Query<&mut Window>, mut q: Query<(&UnitSize, &mut Transform)>) {
    let window = windows.single_mut();
    for (sprite_size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.width / MAZE_WIDTH as f32 * window.width() as f32,
            sprite_size.height / MAZE_HEIGHT as f32 * window.height() as f32,
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
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, MAZE_WIDTH as f32),
            convert(pos.y as f32, window.height() as f32, MAZE_HEIGHT as f32),
            0.0,
        );
    }
}
