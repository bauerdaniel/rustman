//
// Daniel Bauer (bauerda@pm.me)
//

use bevy::prelude::*;

use super::game::*;
use super::states::*;

pub const SOUND_DURATION_START: f32 = 5.;
pub const SOUND_DURATION_AMBIENT_SIREN: f32 = 0.45;
pub const SOUND_DURATION_AMBIENT_FRIGHT: f32 = 0.55;

pub struct SoundPlugin;

impl Plugin for SoundPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems((
                play_ambient_sound
                    .in_set(OnUpdate(GameState::Running)),
            ))
        ;
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
    if elapsed >= SOUND_DURATION_AMBIENT_FRIGHT && pacman_state.0 == PacmanState::Energized {
        audio.play(asset_server.load("sounds/ambient_fright.ogg"));
        game.elapsed_time_sound = time.elapsed_seconds();
    } else if elapsed >= SOUND_DURATION_AMBIENT_SIREN && pacman_state.0 == PacmanState::Normal {
        let ambient_index = if game.round > 4 { 4 } else { game.round };
        audio.play(asset_server.load(format!("sounds/ambient{}.ogg", ambient_index)));
        game.elapsed_time_sound = time.elapsed_seconds();
    }
}
