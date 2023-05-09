//
// Daniel Bauer (bauerda@pm.me)
//

use bevy::prelude::*;

use crate::game::*;
use crate::maze::*;
use crate::sound::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Start,
    Ready,
    Running,
    Paused,
    Respawn,
    RoundWon,
    NewRound,
    GameOver,
}

pub fn on_enter_state_start(
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    audio.play(asset_server.load("sounds/start.ogg"));
}

pub fn switch_state_to_new_round(
    mut game: ResMut<Game>,
    mut next_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
) {
    if time.elapsed_seconds() - game.elapsed_time_state > 0.05 {
        game.elapsed_time_state = time.elapsed_seconds();
        next_state.set(GameState::NewRound);
    }
}

pub fn switch_state_to_ready(
    mut game: ResMut<Game>,
    mut next_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
) {
    if time.elapsed_seconds() - game.elapsed_time_state > 0.05 {
        game.elapsed_time_state = time.elapsed_seconds();
        next_state.set(GameState::Ready);
    }
}

pub fn switch_state_to_running(
    mut game: ResMut<Game>,
    mut next_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
) {
    if time.elapsed_seconds() - game.elapsed_time_state > SOUND_DURATION_START {
        game.elapsed_time_state = time.elapsed_seconds();
        next_state.set(GameState::Running);
    }
}

pub fn switch_state_to_respawn_or_game_over(
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

pub fn switch_state_to_round_won(
    mut game: ResMut<Game>,
    mut next_game_state: ResMut<NextState<GameState>>,
    query_dot: Query<&Dot>,
    query_energizer: Query<&Energizer>,
    time: Res<Time>,
) {
    if query_dot.is_empty() && query_energizer.is_empty() {
        game.elapsed_time_state = time.elapsed_seconds();
        next_game_state.set(GameState::RoundWon);
        game.round += 1;
    }
}
