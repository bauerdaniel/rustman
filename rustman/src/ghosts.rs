//
// Daniel Bauer (bauerda@pm.me)
//

use bevy::prelude::*;

use super::collision::*;
use super::game_state::*;
use super::game::*;
use super::unit::*;

const GHOST_SPEED_NORMAL: f32 = 400.;
const GHOST_SPEED_FRIGHTENED: f32 = 300.;
const GHOST_SPEED_ROUND_INCREASE: f32 = 25.;
const GHOST_SPEED_MAX: f32 = 500.;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum GhostId {
    Blinky,
    Pinky,
    Inky,
    Clyde,
    Frightened,
    #[allow(dead_code)]
    FrightenedBlink,
}

impl GhostId {
    pub fn get_id(&self) -> usize {
        match self {
            GhostId::Blinky => 0,
            GhostId::Pinky => 1,
            GhostId::Inky => 2,
            GhostId::Clyde => 3,
            GhostId::Frightened => 4,
            GhostId::FrightenedBlink => 5,
        }
    }

    pub fn get_sprite_index(&self) -> usize {
        match self {
            GhostId::Blinky => 0,
            GhostId::Pinky => 2,
            GhostId::Inky => 4,
            GhostId::Clyde => 6,
            GhostId::Frightened => 8,
            GhostId::FrightenedBlink => 10,
        }
    }

    pub fn get_start_pos(&self) -> UnitPosition {
        UnitPosition { x: 1213 + (self.get_id() * 110) as i32, y: 613 }
    }
}

pub struct GhostsPlugin;

impl Plugin for GhostsPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(FixedTime::new_from_secs(0.003))
            .add_systems((
                move_ghosts_out.in_schedule(CoreSchedule::FixedUpdate),
                ghosts_movement.in_set(OnUpdate(GameState::Running)),
                animate_ghosts,
            ))
        ;
    }
}

#[derive(Component)]
pub struct Ghost {
    pub ghost_id: GhostId,
    pub current_direction: UnitDirection,
    pub animation_count: usize,
    pub spawn_time: f32,
    pub is_moved_out: bool,
    pub is_frightened: bool,
    pub movement_time: f32,
}

impl Ghost {
    pub fn new(ghost_id: GhostId, spawn_time: f32) -> Self {
        Self {
            ghost_id,
            current_direction: UnitDirection::random(),
            animation_count: 0,
            spawn_time,
            is_moved_out: false,
            is_frightened: false,
            movement_time: 0.,
        }
    }
}

fn load_ghost_sprite(
    ghost_id: GhostId,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) -> SpriteSheetBundle {

    let texture_atlas = TextureAtlas::from_grid(
        asset_server.load("sprites/ghosts.png"),
        Vec2::new(UNIT_SIZE as f32, UNIT_SIZE as f32),
        2,
        5,
        None,
        None
    );

    SpriteSheetBundle {
        texture_atlas: texture_atlases.add(texture_atlas),
        sprite: TextureAtlasSprite::new(ghost_id.get_sprite_index()),
        ..default()
    }
}

fn spawn_ghost(
    ghost_id: GhostId,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    time: &Res<Time>,
) {
    commands.spawn((
        Ghost::new(ghost_id.clone(), time.elapsed_seconds() + 5. * ghost_id.get_id() as f32),
        ghost_id.get_start_pos(),
        UnitScale::square(0.95),
        load_ghost_sprite(ghost_id, asset_server, texture_atlases),
    ));
}

pub fn spawn_ghosts(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    time: Res<Time>,
) {
    spawn_ghost(GhostId::Blinky, &mut commands, &asset_server, &mut texture_atlases, &time);
    spawn_ghost(GhostId::Pinky, &mut commands, &asset_server, &mut texture_atlases, &time);
    spawn_ghost(GhostId::Inky, &mut commands, &asset_server, &mut texture_atlases, &time);
    spawn_ghost(GhostId::Clyde, &mut commands, &asset_server, &mut texture_atlases, &time);
}

pub fn despawn_ghosts(
    mut commands: Commands,
    query_ghosts: Query<Entity, With<Ghost>>,
) {
    for entity in query_ghosts.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn ghosts_movement(
    game: Res<Game>,
    mut query_ghosts: Query<(&mut Ghost, &mut UnitPosition)>,
    game_state: Res<State<GameState>>,
    time: Res<Time>,
) {
    if game_state.0 != GameState::Running { return; }

    for (mut ghost, mut ghost_pos) in query_ghosts.iter_mut() {
        
        if !ghost.is_moved_out || ghost.current_direction == UnitDirection::None { continue; }

        // Determine direction
        let mut next_random_direction = UnitDirection::random();
        while next_random_direction == ghost.current_direction.opposite() {
            next_random_direction = UnitDirection::random();
        }

        while !unit_can_move_in_direction(&ghost_pos, ghost.current_direction) {
            ghost.current_direction = UnitDirection::random();
        }

        // Move ghost forward
        let ghost_speed = if ghost.is_frightened {
            GHOST_SPEED_FRIGHTENED
        } else {
            let round_speed = GHOST_SPEED_NORMAL + game.round as f32 * GHOST_SPEED_ROUND_INCREASE;
            if round_speed > GHOST_SPEED_MAX { GHOST_SPEED_MAX } else { round_speed }
        };

        let pixel_speed = (time.delta_seconds() * ghost_speed) as i32;
        for _ in 0..pixel_speed {
            if unit_can_move_in_direction(&ghost_pos, next_random_direction) {
                ghost.current_direction = next_random_direction;
                ghost_pos.move_in_direction(ghost.current_direction);
            } else if unit_can_move_in_direction(&ghost_pos, ghost.current_direction) {
                ghost_pos.move_in_direction(ghost.current_direction);
            }
        }
    }
}

pub fn move_ghosts_out(
    mut query_ghosts: Query<(&mut Ghost, &mut UnitPosition)>,
    time: Res<Time>,
) {
    for (mut ghost, mut ghost_pos) in query_ghosts.iter_mut() {

        if ghost.spawn_time > time.elapsed_seconds() {
            continue;
        }

        if !ghost.is_moved_out {
            if ghost_pos.y < 713 {
                ghost_pos.move_in_direction(UnitDirection::Up);
            } else if ghost_pos.y >= 713 && ghost_pos.x > 1380 {
                ghost_pos.move_in_direction(UnitDirection::Left);
            } else if ghost_pos.y >= 713 && ghost_pos.x < 1380 {
                ghost_pos.move_in_direction(UnitDirection::Right);
            } else if ghost_pos.y < 883 {
                ghost_pos.move_in_direction(UnitDirection::Up);
            } else {
                ghost.is_moved_out = true;
            }
        }
    }
}

pub fn animate_ghosts(
    mut query_ghosts: Query<(&mut Ghost, &mut TextureAtlasSprite)>,
) {
    for (mut ghost, mut sprite) in query_ghosts.iter_mut() {
        sprite.index = if !ghost.is_frightened {
            ghost.ghost_id.get_sprite_index() + sprite.index % 2
        } else {
            GhostId::Frightened.get_sprite_index() + sprite.index % 2
        };

        // Update sprite index
        ghost.animation_count += 1;
        if ghost.animation_count % 30 == 0 { 
            if sprite.index % 2 == 0 { sprite.index += 1; } else { sprite.index -= 1; }
            ghost.animation_count = 0;
        }
    }
}
