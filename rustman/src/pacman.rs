//
// Daniel Bauer (bauerda@pm.me)
//

use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use super::unit::*;
use super::collision::*;
use super::game::*;
use super::maze::*;

const FIXED_TIMESTEP: f32 = 0.0025;
const PACMAN_SIZE: u32 = 100;

pub struct PacmanPlugin;

impl Plugin for PacmanPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(FixedTime::new_from_secs(FIXED_TIMESTEP))
            .add_system(pacman_movement_input.before(pacman_movement))
            .add_system(pacman_movement.in_schedule(CoreSchedule::FixedUpdate))
            .add_system(pacman_eats_dot.after(pacman_movement))
        ;
    }
}

#[derive(Component)]
pub struct Pacman {
    pub current_direction: UnitDirection,
    pub next_direction: UnitDirection,
    pub eaten_ghosts: u32,
}

pub fn spawn_pacman(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Pacman {
                current_direction: UnitDirection::Left,
                next_direction: UnitDirection::None,
                eaten_ghosts: 0
            },
            SpriteBundle {
                texture: asset_server.load("pacman.png"),
                ..default()
            },
            UnitPosition { x: 1380, y: 150 },
            UnitSize::square(0.9)
        ));
}

pub fn pacman_movement_input(keyboard_input: Res<Input<KeyCode>>, mut q: Query<&mut Pacman>) {
    if let Some(mut pacman) = q.iter_mut().next() {  
        if keyboard_input.pressed(KeyCode::Left) {
            pacman.next_direction = UnitDirection::Left
        } else if keyboard_input.pressed(KeyCode::Down) {
            pacman.next_direction = UnitDirection::Down
        } else if keyboard_input.pressed(KeyCode::Up) {
            pacman.next_direction = UnitDirection::Up
        } else if keyboard_input.pressed(KeyCode::Right) {
            pacman.next_direction = UnitDirection::Right
        };
    }
}

pub fn pacman_movement(mut q: Query<(&mut UnitPosition, &mut Pacman)>) {
    if let Some((mut pos, mut pacman)) = q.iter_mut().next() {
        let mut new_pos = pos.clone();

        match &pacman.next_direction {
            UnitDirection::Left => new_pos.x -= 1,
            UnitDirection::Right => new_pos.x += 1,
            UnitDirection::Up => new_pos.y += 1,
            UnitDirection::Down => new_pos.y -= 1,
            _ => {}
        };

        if check_in_map(new_pos.x, new_pos.y, PACMAN_SIZE)
            && !check_for_collisions(new_pos.x, new_pos.y, PACMAN_SIZE) {
            pos.x = new_pos.x;
            pos.y = new_pos.y;
            pacman.current_direction = pacman.next_direction;
            
        } else {
            let mut new_pos = pos.clone();
            match &pacman.current_direction {
                UnitDirection::Left => new_pos.x -= 1,
                UnitDirection::Right => new_pos.x += 1,
                UnitDirection::Up => new_pos.y += 1,
                UnitDirection::Down => new_pos.y -= 1,
                _ => {}
            };

            if check_in_map(new_pos.x, new_pos.y, PACMAN_SIZE)
                && !check_for_collisions(new_pos.x, new_pos.y, PACMAN_SIZE) {
                pos.x = new_pos.x;
                pos.y = new_pos.y;
            }
        }
    }
}

pub fn pacman_eats_dot(
    mut commands: Commands,
    mut game: ResMut<Game>,
    mut q_p: Query<(&Transform, &mut Pacman)>,
    q_d: Query<(Entity, &Transform), With<Dot>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>
) {
    if let Some((pac_pos, mut pacman)) = q_p.iter_mut().next() {
        for (dot_entity, dot_pos) in q_d.iter() {
            if let Some(_) = collide(pac_pos.translation, Vec2 { x: 2., y: 2. }, dot_pos.translation, Vec2 { x: 2., y: 2. }) {
                
                pacman.eaten_ghosts += 1;
                game.points += 10;
                commands.entity(dot_entity).despawn();

                if pacman.eaten_ghosts % 2 == 0 {
                    let music = asset_server.load("sounds/eat2.ogg");
                    audio.play(music);
                } else {
                    let music = asset_server.load("sounds/eat.ogg");
                    audio.play(music);
                }   
            }
        }
    }
}
