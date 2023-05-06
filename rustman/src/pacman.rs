//
// Daniel Bauer (bauerda@pm.me)
//

use bevy::prelude::*;

use super::unit::*;
use super::collision::*;
use super::game_state::*;

const PACMAN_SPEED: f32 = 0.0025;

pub struct PacmanPlugin;

impl Plugin for PacmanPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(FixedTime::new_from_secs(PACMAN_SPEED))
            .add_startup_system(spawn_pacman)
            .add_systems((
                pacman_movement_input.before(pacman_movement),
                pacman_movement.in_schedule(CoreSchedule::FixedUpdate),
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
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle, Vec2::new(UNIT_SIZE as f32, UNIT_SIZE as f32), 3, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn((
            Pacman {
                current_direction: UnitDirection::Left,
                next_direction: UnitDirection::Left,
                eaten_ghosts: 0,
                animation_count: 0,
            },
            UnitName("Pacman".to_string()),
            UnitPosition { x: 1380, y: 150 },
            UnitScale::square(0.95),
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(0),
                ..default()
            },
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
    state: Res<State<GameState>>,
    mut q: Query<(&mut Pacman, &mut UnitPosition, &mut TextureAtlasSprite, &mut Transform)>,
) {
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

    if state.0 != GameState::Running { return; }

    if let Some((
        mut pacman,
        mut pos,
        sprite,
        transform,
    )) = q.iter_mut().next() {
        if unit_can_move_in_direction(&pos, pacman.next_direction) {
            pacman.current_direction = pacman.next_direction;
            pos.move_in_direction(pacman.current_direction);
            animate(pacman, sprite, transform);
        } else if unit_can_move_in_direction(&pos, pacman.current_direction) {
            pos.move_in_direction(pacman.current_direction);
            animate(pacman, sprite, transform);
        }
    }
}
