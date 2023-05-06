//
// Daniel Bauer (bauerda@pm.me)
//

use bevy::prelude::*;

use super::maze::*;
use super::pacman::*;
use super::ghosts::*;
use super::interactions::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Game {
                round: 1,
                points: 0
            })
            .add_startup_systems((
                setup_maze,
                spawn_points
            ))
            .add_plugin(PacmanPlugin)
            .add_plugin(GhostsPlugin)
            .add_plugin(InteractionsPlugin)
        ;
    }
}

#[derive(Resource)]
pub struct Game {
    pub round: u32,
    pub points: u32,
}

