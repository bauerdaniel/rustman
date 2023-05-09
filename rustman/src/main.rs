//
// Daniel Bauer (bauerda@pm.me)
//

mod collision;
mod states;
mod game;
mod ghosts;
mod input;
mod maze;
mod pacman;
mod scaling;
mod sound;
mod ui;
mod unit;

use bevy::{
    prelude::*,
    window::PresentMode,
    render::color::Color,
};

use game::GamePlugin;
use ghosts::GhostsPlugin;
use maze::MazePlugin;
use pacman::PacmanPlugin;
use scaling::ScalingPlugin;
use sound::SoundPlugin;
use states::StatesPlugin;
use ui::UiPlugin;
use input::InputPlugin;

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
        .add_plugin(StatesPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(MazePlugin)
        .add_plugin(PacmanPlugin)
        .add_plugin(GhostsPlugin)
        .add_plugin(InputPlugin)
        .add_plugin(ScalingPlugin)
        .add_plugin(SoundPlugin)
        .add_plugin(UiPlugin)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
