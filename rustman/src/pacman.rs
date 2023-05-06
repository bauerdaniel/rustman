//
// Daniel Bauer (bauerda@pm.me)
//

use bevy::prelude::*;

use super::unit::*;
use super::collision::*;
use super::game_state::*;
use super::ghosts::*;

const PACMAN_SPEED: f32 = 400.;

pub struct PacmanPlugin;

impl Plugin for PacmanPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<PacmanState>()
            //.insert_resource(FixedTime::new_from_secs(PACMAN_SPEED))
            .add_startup_system(spawn_pacman)
            .add_systems((
                pacman_movement_input.before(pacman_movement),
                pacman_movement,//.in_schedule(CoreSchedule::FixedUpdate),
                pacman_energized.in_set(OnUpdate(PacmanState::Energized)),
                on_pacman_state_normal.in_schedule(OnEnter(PacmanState::Normal)),
            ))
        ;
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum PacmanState {
    #[default]
    Normal,
    Energized,
}

#[derive(Component)]
pub struct Pacman {
    pub current_direction: UnitDirection,
    pub next_direction: UnitDirection,
    pub eaten_points: u32,
    pub eaten_ghosts: u32,
    pub animation_count: u32,
    pub start_time_energized: f32,
    pub sound_play_count: u32,
}

pub fn spawn_pacman(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {

    let texture_handle = asset_server.load("sprites/pacman.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle, Vec2::new(UNIT_SIZE as f32, UNIT_SIZE as f32), 3, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn((
            Pacman {
                current_direction: UnitDirection::Left,
                next_direction: UnitDirection::Left,
                eaten_points: 0,
                eaten_ghosts: 0,
                animation_count: 0,
                start_time_energized: 0.,
                sound_play_count: 0,
            },
            UnitName("Pacman".to_string()),
            UnitPosition { x: 1380, y: 150 },
            UnitScale::square(0.95),
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(0),
                ..default()
            },
        ));
}

pub fn pacman_movement_input(keyboard_input: Res<Input<KeyCode>>, mut q: Query<&mut Pacman>) {
    if let Some(mut pacman) = q.iter_mut().next() {  
        if keyboard_input.pressed(KeyCode::Left) {
            pacman.next_direction = UnitDirection::Left
        } else if keyboard_input.pressed(KeyCode::Down) {
            pacman.next_direction = UnitDirection::Down
        } else if keyboard_input.pressed(KeyCode::Up) {
            pacman.next_direction = UnitDirection::Up
        } else if keyboard_input.pressed(KeyCode::Right) {
            pacman.next_direction = UnitDirection::Right
        };
    }
}

pub fn pacman_movement(
    state: Res<State<GameState>>,
    mut query_pacman: Query<(&mut Pacman, &mut UnitPosition, &mut TextureAtlasSprite, &mut Transform)>,
    time: Res<Time>,
) {
    fn animate(
        pacman: &mut Mut<Pacman>,
        sprite: &mut Mut<TextureAtlasSprite>,
        transform: &mut Mut<Transform>,
    ) {
        pacman.animation_count += 1;
        if pacman.animation_count % 30 != 0 { return; }
        sprite.index = if sprite.index == 2 { 0 } else { sprite.index + 1 };

        transform.rotation = Quat::from_rotation_z(
            match pacman.current_direction {
                UnitDirection::Up => f32::to_radians(270.0),
                UnitDirection::Right => f32::to_radians(180.0),
                UnitDirection::Down => f32::to_radians(90.0),
                _ => f32::to_radians(0.)
        });
        
        pacman.animation_count = 0;
    }

    if state.0 != GameState::Running { return; }

    if let Some((
        mut pacman,
        mut pos,
        mut sprite,
        mut transform,
    )) = query_pacman.iter_mut().next() {
        // Move pacman forward
        let pixel_speed = (time.delta_seconds() * PACMAN_SPEED )as i32;
        for _ in 0..pixel_speed {
            if unit_can_move_in_direction(&pos, pacman.next_direction) {
                pacman.current_direction = pacman.next_direction;
                //pos.move_in_direction(pacman.current_direction);
                //animate(pacman, sprite, transform);
            } else if unit_can_move_in_direction(&pos, pacman.current_direction) {
                //pos.move_in_direction(pacman.current_direction);
                //animate(pacman, sprite, transform);
            } else {
                break;
            }
            pos.move_in_direction(pacman.current_direction);
            animate(&mut pacman, &mut sprite, &mut transform);
        }
    }
}

fn pacman_energized(
    time: Res<Time>,
    mut query_pacman: Query<&mut Pacman>,
    mut next_pacman_state: ResMut<NextState<PacmanState>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    let mut pacman = query_pacman.single_mut();
    
    let elapsed_since_energized = time.elapsed_seconds() - pacman.start_time_energized;
    
    if elapsed_since_energized > 10. {
        next_pacman_state.set(PacmanState::Normal);
    }
    else if elapsed_since_energized > 0.50 * pacman.sound_play_count as f32 {
        audio.play(asset_server.load("sounds/ambient_fright.ogg"));
        pacman.sound_play_count += 1;
    }    
}

fn on_pacman_state_normal(
    mut query_ghosts: Query<&mut Ghost>,
) {
   for mut ghost in query_ghosts.iter_mut() {
        ghost.is_frightened = false;
   } 
}
