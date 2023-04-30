//
// Daniel Bauer (bauerda@pm.me)
//

use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use super::unit::*;

pub const MAZE_WIDTH: u32 = 3700;
pub const MAZE_HEIGHT: u32 = 1230;

pub const DOTS_HORIZONTAL: u32 = 52;
pub const DOTS_VERTICAL: u32 = 15;

const DOT_COLOR: Color = Color::rgba(1., 0.666, 0.643, 1.);

pub fn setup_maze(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("maze.png"),
            ..default()
        },
        UnitSize::square(1.)
    ));
}

pub fn spawn_dots(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for i in 0..DOTS_VERTICAL {
        for j in 0..DOTS_HORIZONTAL {

            let x = 150. + j as f32 * 66.66;
            let y = 150. + i as f32 * 66.66;

            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(10.).into()).into(),
                    material: materials.add(ColorMaterial::from(DOT_COLOR)),
                    //transform: Transform::from_translation(Vec3::new(x, y, 0.)),
                    ..default()
                },
                UnitPosition { x: x as i32, y: y as i32 },
                UnitSize::square(1.)
            ));
        }
    }
}
