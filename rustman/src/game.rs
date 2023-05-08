//
// Daniel Bauer (bauerda@pm.me)
//

use bevy::prelude::*;

use super::collision::*;
use super::game_state::*;
use super::maze::*;
use super::pacman::*;
use super::ghosts::*;
use super::unit::*;

const DURATION_SOUND_START: f32 = 5.;
const DURATION_SOUND_AMBIENT_SIREN: f32 = 0.45;
const DURATION_SOUND_AMBIENT_FRIGHT: f32 = 0.55;

const POINTS_DOT: u32 = 10;
const POINTS_ENERGIZER: u32 = 50;
const POINTS_GHOST: u32 = 200;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Game {
                round: 1,
                points: 0,
                lifes: 3,
                elapsed_time_state: 0.,
                elapsed_time_sound: 0.,
            })
            .add_startup_systems((
                setup_maze,
            ))
            .add_plugin(PacmanPlugin)
            .add_plugin(GhostsPlugin)
            .add_systems((
                play_start_music.in_schedule(OnEnter(GameState::Start)),
                switch_state_to_new_round.in_set(OnUpdate(GameState::Start)),
            ))
            .add_systems((
                despawn_pacman.in_schedule(OnEnter(GameState::NewRound)),
                spawn_pacman.in_schedule(OnEnter(GameState::NewRound)).after(despawn_pacman),
                despawn_ghosts.in_schedule(OnEnter(GameState::NewRound)),
                spawn_ghosts.in_schedule(OnEnter(GameState::NewRound)).after(despawn_ghosts),
                spawn_points.in_schedule(OnEnter(GameState::NewRound)),
                switch_state_to_ready.in_set(OnUpdate(GameState::NewRound)),
            ))
            .add_systems((
                switch_state_to_running.in_set(OnUpdate(GameState::Ready)),
            ))
            .add_systems((
                play_ambient_sound.in_set(OnUpdate(GameState::Running)),
                pacman_eats_dot.in_set(OnUpdate(GameState::Running)),
                pacman_eats_energizer.in_set(OnUpdate(GameState::Running)),
                pacman_eats_ghost.in_set(OnUpdate(GameState::Running)),
                ghost_eats_pacman.in_set(OnUpdate(GameState::Running)),
                round_complete.in_set(OnUpdate(GameState::Running)),
            ))
            .add_systems((
                despawn_pacman.in_schedule(OnEnter(PacmanState::Dead)),
                despawn_ghosts.in_schedule(OnEnter(PacmanState::Dead)),
                respawn_or_game_over.in_schedule(OnEnter(PacmanState::Dead)),
                spawn_pacman.in_schedule(OnEnter(GameState::Respawn)),
                spawn_ghosts.in_schedule(OnEnter(GameState::Respawn)),
                switch_state_to_ready.in_set(OnUpdate(GameState::Respawn)),
            ))
        ;
    }
}

#[derive(Resource)]
pub struct Game {
    pub round: u32,
    pub points: u32,
    pub lifes: u32,
    pub elapsed_time_state: f32,
    pub elapsed_time_sound: f32,
}

fn play_start_music(
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    let music = asset_server.load("sounds/start.ogg");
    audio.play(music);
}

fn switch_state_to_new_round(
    mut game: ResMut<Game>,
    mut next_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
) {
    if time.elapsed_seconds() - game.elapsed_time_state > 0.05 {
        game.elapsed_time_state = time.elapsed_seconds();
        next_state.set(GameState::NewRound);
    }
}

fn switch_state_to_ready(
    mut game: ResMut<Game>,
    mut next_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
) {
    if time.elapsed_seconds() - game.elapsed_time_state > 0.05 {
        game.elapsed_time_state = time.elapsed_seconds();
        next_state.set(GameState::Ready);
    }
}

fn switch_state_to_running(
    mut game: ResMut<Game>,
    mut next_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
) {
    if time.elapsed_seconds() - game.elapsed_time_state > DURATION_SOUND_START {
        game.elapsed_time_state = time.elapsed_seconds();
        next_state.set(GameState::Running);
    }
}

fn pacman_eats_dot(
    mut commands: Commands,
    mut game: ResMut<Game>,
    mut query_pacman: Query<(&mut Pacman, &UnitPosition)>,
    query_dot: Query<(Entity, &UnitPosition), With<Dot>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    if let Some((mut pac, pac_pos)) = query_pacman.iter_mut().next() {
        for (dot_entity, dot_pos) in query_dot.iter() {
            if units_collide(&pac_pos, UNIT_HITBOX_SIZE, &dot_pos, UNIT_HITBOX_SIZE) {
                // Add game points
                game.points += POINTS_DOT;
                
                // Despawn dot
                commands.entity(dot_entity).despawn();

                // Play eat sound
                pac.eaten_points += 1;
                audio.play(asset_server.load(
                    if pac.eaten_points % 2 == 0 { "sounds/eat2.ogg"} else { "sounds/eat.ogg" } ));
            }
        }
    }
}

pub fn pacman_eats_energizer(
    mut commands: Commands,
    mut game: ResMut<Game>,
    mut query_pacman: Query<(&mut Pacman, &UnitPosition)>,
    mut query_ghosts: Query<&mut Ghost>,
    query_energizer: Query<(Entity, &UnitPosition), With<Energizer>>,
    mut next_pacman_state: ResMut<NextState<PacmanState>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    time: Res<Time>,
) {
    if let Some((mut pac, pac_pos)) = query_pacman.iter_mut().next() {
        for (energizer_entity, energizer_pos) in query_energizer.iter() {
            if units_collide(&pac_pos, UNIT_HITBOX_SIZE, &energizer_pos, UNIT_HITBOX_SIZE) {
                // Add game points
                game.points += POINTS_ENERGIZER;

                // Despawn energizer
                commands.entity(energizer_entity).despawn();

                // Play eat sound
                pac.eaten_points += 1;
                audio.play(asset_server.load(
                    if pac.eaten_points % 2 == 0 { "sounds/eat2.ogg"} else { "sounds/eat.ogg" } ));
                
                // Mark pacman energized
                pac.eaten_ghosts = 0;
                pac.start_time_energized = time.elapsed_seconds();
                next_pacman_state.set(PacmanState::Energized);

                // Mark ghosts as frightened
                for mut ghost in query_ghosts.iter_mut() {
                    ghost.is_frightened = true;
                }
            }
        }
    }
}

pub fn pacman_eats_ghost(
    mut game: ResMut<Game>,
    mut query_pacman: Query<(&mut Pacman, &UnitPosition)>,
    mut query_ghost: Query<(&mut Ghost, &mut UnitPosition), Without<Pacman>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    time: Res<Time>,
) {
    if let Some((mut pacman, pac_pos)) = query_pacman.iter_mut().next() {
        for (mut ghost, mut ghost_pos) in query_ghost.iter_mut() {
            if ghost.is_frightened && units_collide(&pac_pos, UNIT_HITBOX_SIZE, &ghost_pos, UNIT_HITBOX_SIZE) {
                // Play eat ghost sound
                audio.play(asset_server.load("sounds/eat_ghost.ogg"));
                
                // Reset ghost
                *ghost_pos = ghost.ghost_id.get_start_pos();
                ghost.is_moved_out = false;
                ghost.is_frightened = false;
                ghost.spawn_time = time.elapsed_seconds() + 5.;
                
                // Calculate points
                pacman.eaten_ghosts += 1;
                let mut points = POINTS_GHOST;                
                for _ in 0..pacman.eaten_ghosts {
                    points *= 2;
                }
                game.points += points;
            }
        }
    }
}

pub fn ghost_eats_pacman(
    mut next_pacman_state: ResMut<NextState<PacmanState>>,
    mut query_pacman: Query<&UnitPosition, With<Pacman>>,
    mut query_ghost: Query<(&mut Ghost, &UnitPosition)>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    if let Some(pac_pos) = query_pacman.iter_mut().next() {
        for (ghost, ghost_pos) in query_ghost.iter_mut() {
            if !ghost.is_frightened && units_collide(&pac_pos, UNIT_HITBOX_SIZE, &ghost_pos, UNIT_HITBOX_SIZE) {
                // Play pacman death sound
                audio.play(asset_server.load("sounds/death.ogg"));
                // Set pacman state to dead
                next_pacman_state.set(PacmanState::Dead);
            }
        }
    }
}

pub fn respawn_or_game_over(
    mut game: ResMut<Game>,
    mut next_game_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
) {
    game.lifes -= 1;
    game.elapsed_time_state = time.elapsed_seconds();
    if game.lifes < 1 {
        next_game_state.set(GameState::GameOver);
    } else {
        next_game_state.set(GameState::Respawn);
    }
}

pub fn round_complete(
    mut game: ResMut<Game>,
    mut next_game_state: ResMut<NextState<GameState>>,
    query_dot: Query<&Dot>,
    query_energizer: Query<&Energizer>,
    time: Res<Time>,
) {
    if query_dot.is_empty() && query_energizer.is_empty() {
        game.elapsed_time_state = time.elapsed_seconds();
        next_game_state.set(GameState::NewRound);
        game.round += 1;
    }
}

pub fn play_ambient_sound(
    mut game: ResMut<Game>,
    pacman_state: Res<State<PacmanState>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    time: Res<Time>,
) {
    let elapsed = time.elapsed_seconds() - game.elapsed_time_sound;
    if elapsed >= DURATION_SOUND_AMBIENT_FRIGHT && pacman_state.0 == PacmanState::Energized {
        audio.play(asset_server.load("sounds/ambient_fright.ogg"));
        game.elapsed_time_sound = time.elapsed_seconds();
    } else if elapsed >= DURATION_SOUND_AMBIENT_SIREN && pacman_state.0 == PacmanState::Normal {
        let ambient_index = if game.round > 4 { 4 } else { game.round };
        audio.play(asset_server.load(format!("sounds/ambient{}.ogg", ambient_index)));
        game.elapsed_time_sound = time.elapsed_seconds();
    }
}
