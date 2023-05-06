//
// Daniel Bauer (bauerda@pm.me)
//

use bevy::prelude::*;

use super::collision::*;
use super::game_state::*;
use super::pacman::*;
use super::unit::*;

const GHOST_SPEED_NORMAL: f32 = 400.;
const GHOST_SPEED_FRIGHTENED: f32 = 200.;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum GhostId {
    Blinky,
    Pinky,
    Inky,
    Clyde,
    Frightened,
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
            .add_startup_system(spawn_ghosts)
            .add_systems((
                move_ghosts_out.in_schedule(CoreSchedule::FixedUpdate),
                ghosts_movement,//.in_schedule(CoreSchedule::FixedUpdate),
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

fn spawn_ghosts(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    time: Res<Time>,
) {
    spawn_ghost(&mut commands, &asset_server, &mut texture_atlases, &time, GhostId::Blinky);
    spawn_ghost(&mut commands, &asset_server, &mut texture_atlases, &time, GhostId::Pinky);
    spawn_ghost(&mut commands, &asset_server, &mut texture_atlases, &time, GhostId::Inky);
    spawn_ghost(&mut commands, &asset_server, &mut texture_atlases, &time, GhostId::Clyde);
}

fn spawn_ghost(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    time: &Res<Time>,
    ghost_id: GhostId,
) {
    let texture_handle = asset_server.load("sprites/ghosts.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle, Vec2::new(UNIT_SIZE as f32, UNIT_SIZE as f32), 2, 5, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn((
        Ghost {
            ghost_id: ghost_id.clone(),
            current_direction: UnitDirection::random(),
            animation_count: 0,
            spawn_time: time.elapsed_seconds() + 5. * ghost_id.get_id() as f32,
            is_moved_out: false,
            is_frightened: false,
            movement_time: 0.,
        },
        ghost_id.get_start_pos(),
        UnitScale::square(0.95),
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite::new(ghost_id.get_sprite_index()),
            ..default()
        },
    ));
}

pub fn ghosts_movement(
    game_state: Res<State<GameState>>,
    pacman_state: Res<State<PacmanState>>,
    mut query_ghosts: Query<(&mut Ghost, &mut UnitPosition, &mut TextureAtlasSprite)>,
    time: Res<Time>,
) {
    if game_state.0 != GameState::Running { return; }

    for (
        mut ghost,
        mut pos,
        mut sprite,
    ) in query_ghosts.iter_mut() {
        
        if !ghost.is_moved_out || ghost.current_direction == UnitDirection::None { continue; }

        // Determine direction
        while !unit_can_move_in_direction(&pos, ghost.current_direction) {
            ghost.current_direction = UnitDirection::random();
        }

        // Move ghost forward
        let pixel_speed = (time.delta_seconds() * if ghost.is_frightened { GHOST_SPEED_FRIGHTENED } else { GHOST_SPEED_NORMAL }) as i32;
        for _ in 0..pixel_speed {
            if unit_can_move_in_direction(&pos, ghost.current_direction) {
                pos.move_in_direction(ghost.current_direction);
            }
        }

        // Update sprite
        //ghost.animation_count += 1;
        //if ghost.animation_count % 30 == 0 { 
        //    let previous_index = sprite.index;
        //    sprite.index = if pacman_state.0 == PacmanState::Normal { ghost.ghost_id.get_sprite_index() } else { 8 };
        //    if previous_index == sprite.index { sprite.index += 1; }
        //    ghost.animation_count = 0;
        //}
    }
}

pub fn move_ghosts_out(
    mut query_ghosts: Query<(&mut Ghost, &mut UnitPosition)>,
    time: Res<Time>,
) {
    for (mut ghost, mut pos) in query_ghosts.iter_mut() {

        if ghost.spawn_time > time.elapsed_seconds() {
            continue;
        }

        if !ghost.is_moved_out {
            if pos.y < 713 {
                pos.move_in_direction(UnitDirection::Up);
            } else if pos.y >= 713 && pos.x > 1380 {
                pos.move_in_direction(UnitDirection::Left);
            } else if pos.y >= 713 && pos.x < 1380 {
                pos.move_in_direction(UnitDirection::Right);
            } else if pos.y < 883 {
                pos.move_in_direction(UnitDirection::Up);
            } else {
                ghost.is_moved_out = true;
            }
        }
    }
}

pub fn animate_ghosts(
    mut query_ghosts: Query<(&mut Ghost, &mut TextureAtlasSprite)>,
) {
    for (
        mut ghost,
        mut sprite,
    ) in query_ghosts.iter_mut() {
        // Update sprite
        ghost.animation_count += 1;
        if ghost.animation_count % 30 == 0 { 
            let previous_index = sprite.index;
            sprite.index = if !ghost.is_frightened { ghost.ghost_id.get_sprite_index() } else { 8 };
            if previous_index == sprite.index { sprite.index += 1; }
            ghost.animation_count = 0;
        }
    }
}