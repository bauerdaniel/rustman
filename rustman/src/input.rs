//
// Daniel Bauer (bauerda@pm.me)
//

use bevy::prelude::*;

use super::pacman::*;
use super::states::*;
use super::unit::*;

const TOUCH_INPUT_SENSITIVITY: f32 = 30.;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems((
                pause_input,
                pacman_movement_input
                    .in_set(OnUpdate(GameState::Running))
                    .after(pacman_movement_input_touch)
                    .before(pacman_movement),
                pacman_movement_input_touch
                    .in_set(OnUpdate(GameState::Running))
                    .before(pacman_movement_input),
            ))
        ;
    }
}

pub fn pause_input(
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut keys: ResMut<Input<KeyCode>>,
) {
    if keys.pressed(KeyCode::P) {
        if state.0 == GameState::Running {
            next_state.set(GameState::Paused);
        } else if state.0 == GameState::Paused {
            next_state.set(GameState::Running);
        }
        keys.reset(KeyCode::P);
    }
}

pub fn pacman_movement_input(
    keys: Res<Input<KeyCode>>,
    mut query_pacman: Query<&mut Pacman>
) {
    if let Some(mut pac) = query_pacman.iter_mut().next() {  
        if keys.pressed(KeyCode::Left) {
            pac.next_direction = UnitDirection::Left
        } else if keys.pressed(KeyCode::Down) {
            pac.next_direction = UnitDirection::Down
        } else if keys.pressed(KeyCode::Up) {
            pac.next_direction = UnitDirection::Up
        } else if keys.pressed(KeyCode::Right) {
            pac.next_direction = UnitDirection::Right
        };
    }
}

pub fn pacman_movement_input_touch(
    touches: Res<Touches>,
    mut query_pacman: Query<&mut Pacman>
) {
    if let Some(mut pac) = query_pacman.iter_mut().next() { 
        for finger in touches.iter() {
            if finger.start_position().x > finger.position().x
                && finger.start_position().x - finger.position().x > TOUCH_INPUT_SENSITIVITY {
                pac.next_direction = UnitDirection::Left;
            } else if finger.start_position().x < finger.position().x
                && finger.position().x - finger.start_position().x > TOUCH_INPUT_SENSITIVITY {
                pac.next_direction = UnitDirection::Right;
            } else if finger.start_position().y > finger.position().y
                && finger.start_position().y - finger.position().y > TOUCH_INPUT_SENSITIVITY {
                pac.next_direction = UnitDirection::Up;
            } else if finger.start_position().y < finger.position().y
                && finger.position().y - finger.start_position().y > TOUCH_INPUT_SENSITIVITY {
                pac.next_direction = UnitDirection::Down;
            }
        }
    }
}
