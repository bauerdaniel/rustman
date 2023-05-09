//
// Daniel Bauer (bauerda@pm.me)
//

use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use super::collision::*;
use super::game::*;
use super::states::*;
use super::unit::*;

pub const MAZE_WIDTH: u32 = 3700;
pub const MAZE_HEIGHT: u32 = 1233;

const DOTS_HORIZONTAL: u32 = 52;
const DOTS_VERTICAL: u32 = 15;

const DOTS_START_X_Y: f32 = 150.;
const DOTS_SPACING: f32 = 66.66;

const DOT_COLOR: Color = Color::rgba(1., 0.666, 0.643, 1.);

const DOT_RADIUS: f32 = 10.;
const ENERGIZER_RADIUS: f32 = 30.;

pub struct MazePlugin;

impl Plugin for MazePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_systems((
                setup_maze,
            ))
            .add_systems((
                spawn_dots_and_energizers
                    .in_schedule(OnEnter(GameState::NewRound)),
                blink_maze
                    .in_set(OnUpdate(GameState::RoundWon)),
            ))
        ;
    }
}

#[derive(Component)]
pub struct Dot;

#[derive(Component)]
pub struct Energizer;

#[derive(Component)]
pub struct Maze;

fn load_maze_sprite(
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) -> SpriteSheetBundle {
    let texture_atlas = TextureAtlas::from_grid(
        asset_server.load("sprites/maze.png"),
        Vec2::new(MAZE_WIDTH as f32, MAZE_HEIGHT as f32),
        1,
        2,
        None,
        None
    );

    SpriteSheetBundle {
        texture_atlas: texture_atlases.add(texture_atlas),
        sprite: TextureAtlasSprite::new(0),
        ..default()
    }
}

pub fn setup_maze(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn((
        Maze,
        load_maze_sprite(asset_server, texture_atlases),
        UnitPosition { x: (MAZE_WIDTH / 2) as i32, y: (MAZE_HEIGHT / 2) as i32 },
        UnitScale::square(1.)
    ));
}

pub fn spawn_dots_and_energizers(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for i in 0..DOTS_VERTICAL {
        for j in 0..DOTS_HORIZONTAL {
            // Skip some positions where we do not want to have points
            if (i == 0 && (j == 18 || j == 19 ))
                || (i == 7 && (j <= 3 || j >= 48))
                || ((i >= 5 && i <= 10) && (j >= 14 && j <= 23))
                || (i == 11 && (j >= 15 && j <= 23))
                || ((i >= 4 && i <= 13) && j == 23) {
                continue;
            }

            // Calculate the coords to spawn the point
            let x = (DOTS_START_X_Y + j as f32 * DOTS_SPACING) as i32;
            let y = (DOTS_START_X_Y + i as f32 * DOTS_SPACING) as i32;

            // Spawn the point if it don't collide with obstacles
            if !check_for_collisions(x as i32, y as i32, UNIT_HITBOX_SIZE) {
                
                // On the map we have 5 energizers
                if (i == 0 && j == 0)
                    || (i == 0 && j == 51)
                    || (i == 8 && j == 27)
                    || (i == 12 && j == 0)
                    || (i == 12 && j == 51) {
                    spawn_energizer(&mut commands, &mut meshes, &mut materials, x, y);
                } else {
                    spawn_dot(&mut commands, &mut meshes, &mut materials, x, y);
                }
            }
        }
    }
}

fn spawn_dot(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    x: i32,
    y: i32
) {
    commands.spawn((
        Dot,
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(DOT_RADIUS).into()).into(),
            material: materials.add(ColorMaterial::from(DOT_COLOR)),
            ..default()
        },
        UnitPosition { x, y },
        UnitScale::square(1.)
    ));
}

fn spawn_energizer(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    x: i32,
    y: i32
) {
    commands.spawn((
        Energizer,
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(ENERGIZER_RADIUS).into()).into(),
            material: materials.add(ColorMaterial::from(DOT_COLOR)),
            ..default()
        },
        UnitPosition { x, y },
        UnitScale::square(1.)
    ));
}

pub fn blink_maze(
    game: Res<Game>,
    mut query_maze: Query<&mut TextureAtlasSprite, With<Maze>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
) {
    if let Some(mut sprite) = query_maze.iter_mut().next() {

        let elapsed = time.elapsed_seconds() - game.elapsed_time_state;

        if elapsed >= 3. {
            next_game_state.set(GameState::NewRound);
        } else if (elapsed >= 1.4 && elapsed < 1.6)
            || (elapsed >= 1.8 && elapsed < 2.)
            || (elapsed >= 2.2 && elapsed < 2.4)
            || (elapsed >= 2.6 && elapsed < 2.8) {
            sprite.index = 1
        } else {
            sprite.index = 0
        }
    }
}
