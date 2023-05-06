//
// Daniel Bauer (bauerda@pm.me)
//

use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Ready,
    Running,
    Paused,
    GameOver,
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<GameState>()
            .add_system(pause_input)
            .add_systems((
                play_start_music,
                on_game_state_start,
            ).in_schedule(OnEnter(GameState::Ready)))
            .add_system(switch_state_to_running.in_set(OnUpdate(GameState::Ready)))
            .add_systems((
                on_game_state_running,
            ).in_schedule(OnEnter(GameState::Running)))
        ;
    }
}

fn switch_state_to_running(
    time: Res<Time>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if time.elapsed_seconds() > 5. {
        next_state.set(GameState::Running);
    }
}

fn on_game_state_start() {
    print!("Game Ready State");
}

fn on_game_state_running() {
    print!("Game Running State");
}


fn play_start_music(
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    let music = asset_server.load("sounds/start.ogg");
    audio.play(music);
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
