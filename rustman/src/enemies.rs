//
// Daniel Bauer (bauerda@pm.me)
//

use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct UnitColor(String);

pub fn spawn_enemies(commands: &mut Commands) {
    commands.spawn((Enemy, UnitColor("Red".to_string())));
    commands.spawn((Enemy, UnitColor("Pink".to_string())));
    commands.spawn((Enemy, UnitColor("Light Blue".to_string())));
    commands.spawn((Enemy, UnitColor("Orange".to_string())));
}