//
// Daniel Bauer (bauerda@pm.me)
//

use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use super::unit::*;
use super::collision::*;

pub const MAZE_WIDTH: u32 = 3700;
pub const MAZE_HEIGHT: u32 = 1233;

pub const DOTS_HORIZONTAL: u32 = 52;
pub const DOTS_VERTICAL: u32 = 15;

const DOT_COLOR: Color = Color::rgba(1., 0.666, 0.643, 1.);

#[derive(Component)]
pub struct Dot;

#[derive(Component)]
pub struct Energizer;

pub fn setup_maze(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("graphics/maze.png"),
            ..default()
        },
        UnitPosition { x: (MAZE_WIDTH / 2) as i32, y: (MAZE_HEIGHT / 2) as i32 },
        UnitScale::square(1.)
    ));
}

pub fn spawn_points(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for i in 0..DOTS_VERTICAL {
        for j in 0..DOTS_HORIZONTAL {

            if i == 0 && (j == 18 || j == 19 ) { continue; }
            else if i == 7 && (j <= 3 || j >= 48) { continue; }
            else if (i >= 5 && i <= 10) && (j >= 14 && j <= 23) { continue; }
            else if i == 11 && (j >= 15 && j <= 23) { continue; }
            else if (i >= 4 && i <= 13) && j == 23 { continue; }

            let x = (150. + j as f32 * 66.66) as i32;
            let y = (150. + i as f32 * 66.66) as i32;

            if !check_for_collisions(x as i32, y as i32, 10) {
                if i == 0 && j == 0 { spawn_energizer(&mut commands, &mut meshes, &mut materials, x, y); }
                else if i == 0 && j == 51 { spawn_energizer(&mut commands, &mut meshes, &mut materials, x, y); }
                else if i == 8 && j == 27 { spawn_energizer(&mut commands, &mut meshes, &mut materials, x, y); }
                else if i == 12 && j == 0 { spawn_energizer(&mut commands, &mut meshes, &mut materials, x, y); }
                else if i == 12 && j == 51 { spawn_energizer(&mut commands, &mut meshes, &mut materials, x, y); }
                else { spawn_dot(&mut commands, &mut meshes, &mut materials, x, y); }
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
            mesh: meshes.add(shape::Circle::new(10.).into()).into(),
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
            mesh: meshes.add(shape::Circle::new(30.).into()).into(),
            material: materials.add(ColorMaterial::from(DOT_COLOR)),
            ..default()
        },
        UnitPosition { x, y },
        UnitScale::square(1.)
    ));
}
