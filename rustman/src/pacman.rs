//
// Daniel Bauer (bauerda@pm.me)
//

use bevy::prelude::*;

use super::unit::*;
use super::collision::*;
use super::game_state::*;
use super::ghosts::*;

const PACMAN_SPEED: f32 = 450.;
const PACMAN_START_X: i32 = 1380;
const PACMAN_START_Y: i32 = 150;

const DURATION_ENERGIZED: f32 = 8.;

const TOUCH_INPUT_SENSITIVITY: f32 = 30.;

pub struct PacmanPlugin;

impl Plugin for PacmanPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<PacmanState>()
            .add_systems((
                pacman_movement_input_touch.before(pacman_movement_input),
                pacman_movement_input.before(pacman_movement),
                pacman_movement,
                pacman_energized.in_set(OnUpdate(PacmanState::Energized)),
                on_pacman_state_normal.in_schedule(OnEnter(PacmanState::Normal)),
            ))
        ;
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum PacmanState {
    #[default]
    Normal,
    Energized,
    Dead,
}

#[derive(Component)]
pub struct Pacman {
    pub current_direction: UnitDirection,
    pub next_direction: UnitDirection,
    pub eaten_points: u32,
    pub eaten_ghosts: u32,
    pub animation_time: f32,
    pub start_time_energized: f32,
}

impl Pacman {
    pub fn new() -> Self {
        Self {
            current_direction: UnitDirection::Left,
            next_direction: UnitDirection::Left,
            eaten_points: 0,
            eaten_ghosts: 0,
            animation_time: 0.,
            start_time_energized: 0.,
        }
    }
}

fn load_pacman_sprite(
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) -> SpriteSheetBundle {

    let texture_atlas = TextureAtlas::from_grid(
        asset_server.load("sprites/pacman.png"),
        Vec2::new(UNIT_SIZE as f32, UNIT_SIZE as f32),
        3,
        1,
        None,
        None
    );

    SpriteSheetBundle {
        texture_atlas: texture_atlases.add(texture_atlas),
        sprite: TextureAtlasSprite::new(0),
        ..default()
    }
}

pub fn spawn_pacman(
    mut commands: Commands,
    mut next_pacman_state: ResMut<NextState<PacmanState>>,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands
        .spawn((
            Pacman::new(),
            UnitPosition { x: PACMAN_START_X, y: PACMAN_START_Y },
            UnitScale::square(0.95),
            load_pacman_sprite(asset_server, texture_atlases),
        ));

    next_pacman_state.set(PacmanState::Normal);
}

pub fn despawn_pacman(
    mut commands: Commands,
    mut query_pacman: Query<Entity, With<Pacman>>,
) {
    if let Some(pacman_entity) = query_pacman.iter_mut().next() {
        commands.entity(pacman_entity).despawn();
    }
}

pub fn pacman_movement_input(
    keys: Res<Input<KeyCode>>,
    mut query_pacman: Query<&mut Pacman>
) {
    if let Some(mut pac) = query_pacman.iter_mut().next() {  
        if keys.pressed(KeyCode::Left) {
            pac.next_direction = UnitDirection::Left
        } else if keys.pressed(KeyCode::Down) {
            pac.next_direction = UnitDirection::Down
        } else if keys.pressed(KeyCode::Up) {
            pac.next_direction = UnitDirection::Up
        } else if keys.pressed(KeyCode::Right) {
            pac.next_direction = UnitDirection::Right
        };
    }
}

fn pacman_movement_input_touch(
    touches: Res<Touches>,
    mut query_pacman: Query<&mut Pacman>
) {
    if let Some(mut pac) = query_pacman.iter_mut().next() { 
        for finger in touches.iter() {
            if finger.start_position().x > finger.position().x
                && finger.start_position().x - finger.position().x > TOUCH_INPUT_SENSITIVITY {
                pac.next_direction = UnitDirection::Left;
            } else if finger.start_position().x < finger.position().x
                && finger.position().x - finger.start_position().x > TOUCH_INPUT_SENSITIVITY {
                pac.next_direction = UnitDirection::Right;
            } else if finger.start_position().y > finger.position().y
                && finger.start_position().y - finger.position().y > TOUCH_INPUT_SENSITIVITY {
                pac.next_direction = UnitDirection::Up;
            } else if finger.start_position().y < finger.position().y
                && finger.position().y - finger.start_position().y > TOUCH_INPUT_SENSITIVITY {
                pac.next_direction = UnitDirection::Down;
            }
        }
    }
}

pub fn pacman_movement(
    state: Res<State<GameState>>,
    mut query_pacman: Query<(&mut Pacman, &mut UnitPosition, &mut TextureAtlasSprite, &mut Transform)>,
    time: Res<Time>,
) {
    fn animate(
        pacman: &mut Mut<Pacman>,
        sprite: &mut Mut<TextureAtlasSprite>,
        transform: &mut Mut<Transform>,
        elapsed_startup: f32,
    ) {
        let elapsed = elapsed_startup - pacman.animation_time;
        if elapsed < 0.05 { return; }
        sprite.index = if sprite.index == 2 { 0 } else { sprite.index + 1 };

        transform.rotation = Quat::from_rotation_z(
            match pacman.current_direction {
                UnitDirection::Up => f32::to_radians(270.0),
                UnitDirection::Right => f32::to_radians(180.0),
                UnitDirection::Down => f32::to_radians(90.0),
                _ => f32::to_radians(0.)
        });
        
        pacman.animation_time = elapsed_startup;
    }

    if state.0 != GameState::Running { return; }

    if let Some((
        mut pacman,
        mut pos,
        mut sprite,
        mut transform,
    )) = query_pacman.iter_mut().next() {
        let pixel_speed = (time.delta_seconds() * PACMAN_SPEED )as i32;
        for _ in 0..pixel_speed {
            if unit_can_move_in_direction(&pos, pacman.next_direction) {
                pacman.current_direction = pacman.next_direction;
            } else if !unit_can_move_in_direction(&pos, pacman.current_direction) {
                break;
            }
            pos.move_in_direction(pacman.current_direction);
            animate(&mut pacman, &mut sprite, &mut transform, time.elapsed_seconds());
        }
    }
}

fn pacman_energized(
    time: Res<Time>,
    query_pacman: Query<&Pacman>,
    mut next_pacman_state: ResMut<NextState<PacmanState>>,
) {
    if let Some(pacman) = query_pacman.iter().next() {
        let elapsed_since_energized = time.elapsed_seconds() - pacman.start_time_energized;
        if elapsed_since_energized > DURATION_ENERGIZED {
            next_pacman_state.set(PacmanState::Normal);
        }
    }
}

fn on_pacman_state_normal(
    mut query_ghosts: Query<&mut Ghost>,
) {
    for mut ghost in query_ghosts.iter_mut() {
        ghost.is_frightened = false;
    } 
}
