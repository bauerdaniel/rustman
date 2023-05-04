//
// Daniel Bauer (bauerda@pm.me)
//

// https://docs.rs/bevy/latest/bevy/index.html
// https://mbuffett.com/posts/bevy-snake-tutorial/

mod collision;
mod maze;
mod pacman;
mod unit;
mod game;
mod game_state;
mod ui;

use bevy::{
    prelude::*,
    window::PresentMode,
    render::color::Color
};

use game::*;
use game_state::GameStatePlugin;
use maze::*;
use pacman::*;
use unit::*;
use ui::UiPlugin;

pub const GAME_WIDTH: u32 = MAZE_WIDTH;
pub const GAME_HEIGHT: u32 = MAZE_HEIGHT;

const FIXED_TIMESTEP: f32 = 0.0025;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Rustman".into(),
                    resolution: (1120., 480.).into(),
                    present_mode: PresentMode::AutoVsync,
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()})
            .set(ImagePlugin::default_nearest()))
        // Plugins
        .add_plugin(GameStatePlugin)
        .add_plugin(GamePlugin)
        .add_plugin(UiPlugin)
        // Setup
        .add_startup_systems((
            setup_camera,
            
            
        ))
        // Scaling
        .add_systems((
            position_translation,
            size_scaling,
            
        ))
        
        // Debug
        .add_event::<CursorMoved>().add_system(cursor_events)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
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

fn cursor_events(
    mut cursor_evr: EventReader<CursorMoved>,
) {
    let window_width: f32 = 1280.;
    let window_height: f32 = 426.;

    let maze_width = GAME_WIDTH as f32;
    let maze_height = GAME_HEIGHT as f32;

    for ev in cursor_evr.iter() {
        let (mouse_x, mouse_y) = (ev.position.x, ev.position.y);

        let scaled_x = mouse_x / window_width * maze_width;
        let scaled_y = mouse_y / window_height * maze_height;

        println!(
            "New cursor position: X: {}, Y: {} (X: {}, Y: {})",
            ev.position.x, ev.position.y, scaled_x, scaled_y
        );
    }
}
