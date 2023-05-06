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
            .add_system(pacman_eats_point)//.after(pacman_movement))
            .add_system(ghost_eats_pacman)//.after(ghosts_movement))
        ;
    }
}

fn pacman_eats_point(
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
                pacman.eaten_ghosts += 1;
                game.points += 10;
                commands.entity(dot_entity).despawn();

                audio.play(asset_server.load(
                    if pacman.eaten_ghosts % 2 == 0 { "sounds/eat2.ogg"} else { "sounds/eat.ogg" } ));
            }
        }
    }
}

pub fn ghost_eats_pacman(
    mut next_state: ResMut<NextState<GameState>>,
    mut query_pacman: Query<(&mut Pacman, &UnitPosition)>,
    mut query_ghost: Query<(Entity, &mut Ghost, &UnitPosition)>,
) {
    if let Some((mut pacman, pac_pos)) = query_pacman.iter_mut().next() {
        for (ghost_entity, ghost, ghost_pos) in query_ghost.iter_mut() {
            if units_collide(&pac_pos, 10, &ghost_pos, 10) {
                next_state.set(GameState::GameOver);
            }
        }
    }
}