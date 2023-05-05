//
// Daniel Bauer (bauerda@pm.me)
//

mod collision;
mod game_state;
mod game;
mod ghosts;
mod interactions;
mod maze;
mod pacman;
mod scaling;
mod ui;
mod unit;

use bevy::{
    prelude::*,
    window::PresentMode,
    render::color::Color
};

use game_state::GameStatePlugin;
use game::GamePlugin;
use scaling::ScalingPlugin;
use ui::UiPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_startup_system(setup_camera)
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
        .add_plugin(GameStatePlugin)
        .add_plugin(GamePlugin)
        .add_plugin(ScalingPlugin)
        .add_plugin(UiPlugin)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
