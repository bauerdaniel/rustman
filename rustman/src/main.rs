//
// Daniel Bauer (bauerda@pm.me)
//

// https://mbuffett.com/posts/bevy-snake-tutorial/

use bevy::{prelude::*, window::PresentMode, render::color::Color};

const MAZE_WIDTH: u32 = 1920 * 60;
const MAZE_HEIGHT: u32 = 1080 * 60;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Size {
    width: f32,
    height: f32,
}

impl Size {
    pub fn square(x: f32) -> Self {
        Self { width: x, height: x }
    }
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct UnitColor(String);

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        //.add_plugins(DefaultPlugins)
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Rustman".into(),
                    resolution: (1920., 1080.).into(),
                    present_mode: PresentMode::AutoVsync,
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()})
            .set(ImagePlugin::default_nearest()))
        .add_startup_system(setup)
        .add_system(player_movement)
        .add_system(position_translation)
        .add_system(size_scaling)
        //.add_systems((
        //    position_translation,
        //    size_scaling))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    setup_camera(&mut commands);
    setup_maze(&mut commands, &asset_server);
    add_player(&mut commands, &asset_server);
    add_enemies(&mut commands);
}

fn setup_camera(commands: &mut Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_maze(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("maze.png"),
        ..default()
    });
}

fn add_player(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands
        .spawn((
            Player,
            SpriteBundle {
                texture: asset_server.load("pacman.png"),
                ..default()
            },
            Position { x: 3, y: 3 },
            Size::square(60.)
        ));
}

fn add_enemies(commands: &mut Commands) {
    commands.spawn((Enemy, UnitColor("Red".to_string())));
    commands.spawn((Enemy, UnitColor("Pink".to_string())));
    commands.spawn((Enemy, UnitColor("Light Blue".to_string())));
    commands.spawn((Enemy, UnitColor("Orange".to_string())));
}

//fn do_something(query: Query<&UnitColor, With<Enemy>>) {
//    for color in &query {
//        println!("hello {}!", color.0);
//    }
//}

/*
fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut head_positions: Query<&mut Transform, With<Player>>,
) {
    for mut transform in head_positions.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            transform.translation.x -= 2.;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            transform.translation.x += 2.;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            transform.translation.y -= 2.;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            transform.translation.y += 2.;
        }
    }
}
*/

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut head_positions: Query<&mut Position, With<Player>>,
) {
    for mut pos in head_positions.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            pos.x -= 1;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            pos.x += 1;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            pos.y -= 1;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            pos.y += 1;
        }
    }
}

fn size_scaling(mut windows: Query<&mut Window>, mut q: Query<(&Size, &mut Transform)>) {
    let window = windows.single_mut();
    for (sprite_size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.width / MAZE_WIDTH as f32 * window.width() as f32,
            sprite_size.height / MAZE_HEIGHT as f32 * window.height() as f32,
            1.0,
        );
    }
}

fn position_translation(mut windows: Query<&mut Window>, mut q: Query<(&Position, &mut Transform)>) {
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
