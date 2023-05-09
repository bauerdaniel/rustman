//
// Daniel Bauer (bauerda@pm.me)
//

use bevy::prelude::*;

use crate::ghosts::*;
use crate::pacman::*;

const DURATION_ENERGIZED: f32 = 8.;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum PacmanState {
    #[default]
    Normal,
    Energized,
    Dead,
    Respawn,
}

pub fn switch_pacman_state_to_energized(
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

pub fn on_enter_pacman_state_normal(
    mut query_ghosts: Query<&mut Ghost>,
) {
    for mut ghost in query_ghosts.iter_mut() {
        ghost.is_frightened = false;
    } 
}
