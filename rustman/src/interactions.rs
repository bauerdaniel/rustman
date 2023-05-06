//
// Daniel Bauer (bauerda@pm.me)
//

use bevy::prelude::*;

use super::collision::*;
use super::game::*;
use super::game_state::*;
use super::maze::*;
use super::pacman::*;
use super::ghosts::*;
use super::unit::*;

pub struct InteractionsPlugin;

impl Plugin for InteractionsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(pacman_eats_dot.in_set(OnUpdate(GameState::Running)))
            .add_system(pacman_eats_energizer.in_set(OnUpdate(GameState::Running)))
            .add_system(ghost_eats_pacman.in_set(OnUpdate(GameState::Running)))//.in_set(OnUpdate(PacmanState::Normal)))
            .add_system(pacman_eats_ghost.in_set(OnUpdate(GameState::Running)))//.in_set(OnUpdate(PacmanState::Energized)))
        ;
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
    if let Some((mut pacman, pac_pos)) = query_pacman.iter_mut().next() {
        for (dot_entity, dot_pos) in query_dot.iter() {

            if units_collide(&pac_pos, 10, &dot_pos, 10) {
                pacman.eaten_points += 1;
                game.points += 10;
                commands.entity(dot_entity).despawn();

                audio.play(asset_server.load(
                    if pacman.eaten_points % 2 == 0 { "sounds/eat2.ogg"} else { "sounds/eat.ogg" } ));
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
    if let Some((mut pacman, pac_pos)) = query_pacman.iter_mut().next() {
        for (energizer_entity, energizer_pos) in query_energizer.iter() {

            if units_collide(&pac_pos, 10, &energizer_pos, 10) {
                // Pacman eats the point
                pacman.eaten_points += 1;
                game.points += 50;
                commands.entity(energizer_entity).despawn();
                audio.play(asset_server.load(
                    if pacman.eaten_points % 2 == 0 { "sounds/eat2.ogg"} else { "sounds/eat.ogg" } ));
                
                // Mark pacman energized
                pacman.eaten_ghosts = 0;
                pacman.sound_play_count = 0;
                pacman.start_time_energized = time.elapsed_seconds();
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

pub fn ghost_eats_pacman(
    mut next_state: ResMut<NextState<GameState>>,
    mut query_pacman: Query<(&mut Pacman, &UnitPosition)>,
    mut query_ghost: Query<(Entity, &mut Ghost, &UnitPosition)>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    if let Some((mut pacman, pac_pos)) = query_pacman.iter_mut().next() {
        for (ghost_entity, ghost, ghost_pos) in query_ghost.iter_mut() {
            if !ghost.is_frightened && units_collide(&pac_pos, 10, &ghost_pos, 10) {
                audio.play(asset_server.load("sounds/death.ogg"));
                next_state.set(GameState::GameOver);
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
                // Play sound
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
