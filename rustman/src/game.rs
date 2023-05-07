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

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Game {
                round: 1,
                points: 0,
                lifes: 3,
            })
            .add_startup_systems((
                setup_maze,
                spawn_points
            ))
            .add_plugin(PacmanPlugin)
            .add_plugin(GhostsPlugin)
            .add_systems((
                pacman_eats_dot.in_set(OnUpdate(GameState::Running)),
                pacman_eats_energizer.in_set(OnUpdate(GameState::Running)),
                pacman_eats_ghost.in_set(OnUpdate(GameState::Running)),
                ghost_eats_pacman.in_set(OnUpdate(GameState::Running)),

                despawn_pacman.in_schedule(OnEnter(PacmanState::Dead)),
                despawn_ghosts.in_schedule(OnEnter(PacmanState::Dead)),
                reset_or_game_over.in_schedule(OnEnter(PacmanState::Dead)),

                spawn_pacman.in_schedule(OnEnter(GameState::Reset)),
                spawn_ghosts.in_schedule(OnEnter(GameState::Reset)),
                reset.in_schedule(OnEnter(GameState::Reset)),
            ))
        ;
    }
}

#[derive(Resource)]
pub struct Game {
    pub round: u32,
    pub points: u32,
    pub lifes: u32,
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
            if units_collide(&pac_pos, 10, &dot_pos, 10) {
                // Add game points
                game.points += 10;
                
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

fn pacman_eats_energizer(
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
            if units_collide(&pac_pos, 10, &energizer_pos, 10) {
                // Add game points
                game.points += 50;

                // Despawn energizer
                commands.entity(energizer_entity).despawn();

                // Play eat sound
                pac.eaten_points += 1;
                audio.play(asset_server.load(
                    if pac.eaten_points % 2 == 0 { "sounds/eat2.ogg"} else { "sounds/eat.ogg" } ));
                
                // Mark pacman energized
                pac.eaten_ghosts = 0;
                pac.sound_play_count = 0;
                pac.start_time_energized = time.elapsed_seconds();
                next_pacman_state.set(PacmanState::Energized);

                // Mark ghosts as frightened
                for mut ghost in query_ghosts.iter_mut() {
                    if ghost.is_moved_out {
                        ghost.is_frightened = true;
                    }
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
            if ghost.is_frightened && units_collide(&pac_pos, 10, &ghost_pos, 10) {
                // Play eat ghost sound
                audio.play(asset_server.load("sounds/eat_ghost.ogg"));
                
                // Reset ghost
                let start_pos = ghost.ghost_id.get_start_pos();
                ghost_pos.x = start_pos.x;
                ghost_pos.y = start_pos.y;
                ghost.is_moved_out = false;
                ghost.is_frightened = false;
                ghost.spawn_time = time.elapsed_seconds() + 5.;
                
                // Calculate points
                pacman.eaten_ghosts += 1;
                let mut points = 100;                
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
            if !ghost.is_frightened && units_collide(&pac_pos, 10, &ghost_pos, 10) {
                // Play pacman death sound
                audio.play(asset_server.load("sounds/death.ogg"));
                // Set pacman state to dead
                next_pacman_state.set(PacmanState::Dead);
            }
        }
    }
}

pub fn reset_or_game_over(
    mut game: ResMut<Game>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    game.lifes -= 1;
    if game.lifes < 1 {
        next_game_state.set(GameState::GameOver);
    } else {
        next_game_state.set(GameState::Reset);
    }
}

pub fn reset(
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    next_game_state.set(GameState::Running);
}
