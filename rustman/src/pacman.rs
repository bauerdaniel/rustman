//
// Daniel Bauer (bauerda@pm.me)
//

use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use super::unit::*;
use super::collision::*;
use super::game::*;
use super::maze::*;
//use super::game_state::*;

const PACMAN_SPEED: f32 = 0.0025;
const PACMAN_SIZE: u32 = 100;

pub struct PacmanPlugin;

impl Plugin for PacmanPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(FixedTime::new_from_secs(PACMAN_SPEED))
            .add_systems((
                pacman_movement_input.before(pacman_movement),
                pacman_movement.in_schedule(CoreSchedule::FixedUpdate),
                pacman_eats_dot.after(pacman_movement)
            ))
        ;
    }
}

#[derive(Component)]
pub struct Pacman {
    pub current_direction: UnitDirection,
    pub next_direction: UnitDirection,
    pub eaten_ghosts: u32,
    pub animation_count: u32,
}

pub fn spawn_pacman(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {

    let texture_handle = asset_server.load("sprites/pacman.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(100.0, 100.0), 3, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn((
            Pacman {
                current_direction: UnitDirection::Left,
                next_direction: UnitDirection::None,
                eaten_ghosts: 0,
                animation_count: 0,
            },
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(0),
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

pub fn pacman_movement(
    //game_state: Res<GameState>,
    mut q: Query<(&mut Pacman, &mut UnitPosition, &mut TextureAtlasSprite, &mut Transform)>,
) {
    fn try_move(dir: &UnitDirection, pos: &mut UnitPosition) -> bool {
        let mut new_pos = pos.clone();
        
        match dir {
            UnitDirection::Left => new_pos.x -= 1,
            UnitDirection::Right => new_pos.x += 1,
            UnitDirection::Up => new_pos.y += 1,
            UnitDirection::Down => new_pos.y -= 1,
            _ => {}
        };

        let can_move = check_in_map(new_pos.x, new_pos.y, PACMAN_SIZE)
            && !check_for_collisions(new_pos.x, new_pos.y, PACMAN_SIZE);

        if can_move {
            pos.x = new_pos.x;
            pos.y = new_pos.y;
        }

        can_move
    }

    fn animate(
        mut pacman: Mut<Pacman>,
        mut sprite: Mut<TextureAtlasSprite>,
        mut transform: Mut<Transform>,
    ) {
        pacman.animation_count += 1;
        if pacman.animation_count % 30 != 0 { return; }
        sprite.index = if sprite.index == 2 { 0 } else { sprite.index + 1 };

        transform.rotation = Quat::from_rotation_z(
            match pacman.current_direction {
                UnitDirection::Up => f32::to_radians(270.0),
                UnitDirection::Right => f32::to_radians(180.0),
                UnitDirection::Down => f32::to_radians(90.0),
                _ => f32::to_radians(0.)
        });
        
        pacman.animation_count = 0;
    }

    if let Some((
        mut pacman,
        mut pos,
        sprite,
        transform,
    )) = q.iter_mut().next() {
        if pacman.next_direction == UnitDirection::None { return; }

        if try_move(&pacman.next_direction, &mut pos) {
            pacman.current_direction = pacman.next_direction;
            animate(pacman, sprite, transform);
        } else {
            if try_move(&pacman.current_direction, &mut pos) {
                animate(pacman, sprite, transform);
            }
        }
    }
}

pub fn pacman_eats_dot(
    mut commands: Commands,
    mut game: ResMut<Game>,
    mut q_p: Query<(&mut Pacman, &Transform)>,
    q_d: Query<(Entity, &Transform), With<Dot>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    if let Some((mut pacman, pac_pos)) = q_p.iter_mut().next() {
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
