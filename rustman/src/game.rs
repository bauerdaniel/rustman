//
// Daniel Bauer (bauerda@pm.me)
//

use bevy::prelude::*;
use super::game_state::*;
use super::maze::*;
use super::pacman::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_systems((
                setup_maze,
                spawn_pacman,
                spawn_dots
            ))
            .add_system(play_start_music.in_schedule(OnEnter(GameState::Start)))
            .add_plugin(PacmanPlugin)
        ;
    }
}

#[derive(Resource)]
pub struct Game {
    pub round: u32,
    pub points: u32,
}

pub fn play_start_music(
    asset_server: Res<AssetServer>,
    audio: Res<Audio>
) {
    let music = asset_server.load("sounds/start.ogg");
    audio.play(music);
}
