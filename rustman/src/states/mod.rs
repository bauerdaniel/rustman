//
// Daniel Bauer (bauerda@pm.me)
//

use bevy::prelude::*;

mod game_state;
mod pacman_state;

pub use game_state::*;
pub use pacman_state::*;

pub struct StatesPlugin;

// This plugin manages the state transition of the game
impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<GameState>()
            .add_state::<PacmanState>()

            // Start State
            .add_systems((
                on_enter_state_start
                    .in_schedule(OnEnter(GameState::Start)),
                switch_state_to_new_round
                    .in_set(OnUpdate(GameState::Start)),
            ))

            // Ready State
            .add_systems((
                switch_state_to_running
                    .in_set(OnUpdate(GameState::Ready)),
            ))

            // Running
            .add_systems((
                switch_state_to_round_won
                    .in_set(OnUpdate(GameState::Running)),
            ))

            // New Round State
            .add_systems((
                switch_state_to_ready
                    .in_set(OnUpdate(GameState::NewRound)),
            ))

            // Respawn State
            .add_systems((
                switch_state_to_respawn_or_game_over
                    .in_schedule(OnEnter(PacmanState::Respawn)),
                switch_state_to_ready
                    .in_set(OnUpdate(GameState::Respawn)),
            ))

            // Pacman States
            .add_systems((
                switch_pacman_state_to_energized
                    .in_set(OnUpdate(GameState::Running))
                    .in_set(OnUpdate(PacmanState::Energized)),
                on_enter_pacman_state_normal
                    .in_set(OnUpdate(GameState::Running))
                    .in_schedule(OnEnter(PacmanState::Normal)),
            ))
        ;
    }
}

