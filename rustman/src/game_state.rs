//
// Daniel Bauer (bauerda@pm.me)
//

use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Start,
    Ready,
    Running,
    Paused,
    Respawn,
    NewRound,
    GameOver,
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<GameState>()
            .add_system(pause_input)
        ;
    }
}

fn pause_input(
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut keys: ResMut<Input<KeyCode>>,
) {
    if keys.pressed(KeyCode::P) {
        next_state.set(if state.0 == GameState::Running { GameState::Paused } else { GameState::Running });
        keys.reset(KeyCode::P);
    }
}
