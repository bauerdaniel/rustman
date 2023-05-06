//
// Daniel Bauer (bauerda@pm.me)
//

use bevy::prelude::*;

use super::collision::*;
use super::game_state::*;
use super::unit::*;

const GHOST_SPEED: f32 = 0.0025;

const GHOST_NAMES: [&str; 4] = [
    "Blinky", // Red
    "Pinky",  // Pink
    "Inky",   // Light Blue
    "Clyde",  // Orange
];

pub struct GhostsPlugin;

impl Plugin for GhostsPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(FixedTime::new_from_secs(GHOST_SPEED))
            .add_startup_system(spawn_ghosts)
            .add_systems((
                ghosts_movement.in_schedule(CoreSchedule::FixedUpdate),
            ))
        ;
    }
}

#[derive(Component)]
pub struct Ghost {
    pub current_direction: UnitDirection,
    pub animation_count: u32,
}

fn spawn_ghosts(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("sprites/ghosts.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle, Vec2::new(UNIT_SIZE as f32, UNIT_SIZE as f32), 2, 4, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    for (i, name) in GHOST_NAMES.iter().enumerate() {
        commands.spawn((
            Ghost {
                current_direction: UnitDirection::random(),
                animation_count: 0,
            }, 
            UnitName(name.to_string()),
            //UnitPosition { x: 1213 + (i * 110) as i32, y: 613 },
            UnitPosition { x: 1213 + (i * 110) as i32, y: 883 },
            UnitScale::square(0.95),
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite::new(i * 2),
                ..default()
            },
        ));
    }
}

pub fn ghosts_movement(
    state: Res<State<GameState>>,
    mut q: Query<(&mut Ghost, &mut UnitPosition, &mut TextureAtlasSprite)>,
) {
    fn animate(
        mut ghost: Mut<Ghost>,
        mut sprite: Mut<TextureAtlasSprite>,
    ) {
        ghost.animation_count += 1;
        if ghost.animation_count % 30 != 0 { return; }
        let idx = sprite.index;
        sprite.index = if idx % 2 == 0 { idx + 1 } else { idx - 1 };
        ghost.animation_count = 0;
    }

    if state.0 != GameState::Running { return; }

    for (
        mut ghost,
        mut pos,
        sprite,
    ) in q.iter_mut() {
        
        if ghost.current_direction == UnitDirection::None { return; }

        while !unit_can_move_in_direction(&pos, ghost.current_direction) {
            ghost.current_direction = UnitDirection::random();
        }

        pos.move_in_direction(ghost.current_direction);
        animate(ghost, sprite);
    }
}
