//
// Daniel Bauer (bauerda@pm.me)
//

use bevy::prelude::*;
use super::unit::*;

#[derive(Component)]
pub struct Pacman {
    pub direction: UnitDirection
}

pub fn spawn_pacman(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Pacman { direction: UnitDirection::Left },
            SpriteBundle {
                texture: asset_server.load("pacman.png"),
                ..default()
            },
            UnitPosition { x: 1380, y: 145 },
            UnitSize::square(0.9)
        ));
}

pub fn pacman_movement_input(keyboard_input: Res<Input<KeyCode>>, mut q: Query<&mut Pacman>) {
    if let Some(mut pacman) = q.iter_mut().next() {
        let dir: UnitDirection = if keyboard_input.pressed(KeyCode::Left) {
            UnitDirection::Left
        } else if keyboard_input.pressed(KeyCode::Down) {
            UnitDirection::Down
        } else if keyboard_input.pressed(KeyCode::Up) {
            UnitDirection::Up
        } else if keyboard_input.pressed(KeyCode::Right) {
            UnitDirection::Right
        } else {
            pacman.direction
        };
        pacman.direction = dir;
    }
}

pub fn pacman_movement(mut q: Query<(&mut UnitPosition, &Pacman)>) {
    if let Some((mut pos, pacman)) = q.iter_mut().next() {
        match &pacman.direction {
            UnitDirection::Left => {
                pos.x -= 1;
            }
            UnitDirection::Right => {
                pos.x += 1;
            }
            UnitDirection::Up => {
                pos.y += 1;
            }
            UnitDirection::Down => {
                pos.y -= 1;
            }
        };
    }
}
