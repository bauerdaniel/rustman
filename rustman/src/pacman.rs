//
// Daniel Bauer (bauerda@pm.me)
//

use bevy::prelude::*;

use super::collision::*;
use super::game::*;
use super::ghosts::*;
use super::maze::*;
use super::sound::*;
use super::states::*;
use super::unit::*;

const PACMAN_SPEED: f32 = 450.;
const PACMAN_START_X: i32 = 1380;
const PACMAN_START_Y: i32 = 150;

pub struct PacmanPlugin;

impl Plugin for PacmanPlugin {
    fn build(&self, app: &mut App) {
        app
            // New Round State
            .add_systems((
                despawn_pacman
                    .in_schedule(OnEnter(GameState::NewRound)),
                spawn_pacman
                    .in_schedule(OnEnter(GameState::NewRound))
                    .after(despawn_pacman),
            ))

            // Running State
            .add_systems((
                pacman_movement
                    .in_set(OnUpdate(GameState::Running)),
                pacman_eats_dot
                    .in_set(OnUpdate(GameState::Running)),
                pacman_eats_energizer
                    .in_set(OnUpdate(GameState::Running)),
                pacman_eats_ghost
                    .in_set(OnUpdate(GameState::Running)),
            ))

            // Respawn State
            .add_systems((
                spawn_pacman
                    .in_schedule(OnEnter(GameState::Respawn)),
            ))

            // Pacman States
            .add_systems((
                spawn_pacman_death_animation
                    .in_schedule(OnEnter(PacmanState::Dead)),
                despawn_pacman
                    .in_schedule(OnEnter(PacmanState::Dead))
                    .after(spawn_pacman_death_animation),
                pacman_death_animation
                    .in_set(OnUpdate(PacmanState::Dead)),
                despawn_pacman_death_animation
                    .in_schedule(OnEnter(PacmanState::Respawn)),
            ))
        ;
    }
}

#[derive(Component)]
pub struct Pacman {
    pub current_direction: UnitDirection,
    pub next_direction: UnitDirection,
    pub eaten_points: u32,
    pub eaten_ghosts: u32,
    pub animation_time: f32,
    pub start_time_energized: f32,
}

impl Pacman {
    pub fn new() -> Self {
        Self {
            current_direction: UnitDirection::Left,
            next_direction: UnitDirection::Left,
            eaten_points: 0,
            eaten_ghosts: 0,
            animation_time: 0.,
            start_time_energized: 0.,
        }
    }
}

#[derive(Component)]
pub struct PacmanDeathAnimation {
    pub start_animation_time: f32,
    pub animation_time: f32,
    pub played_sound: bool,
}

fn load_pacman_death_sprite(
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) -> SpriteSheetBundle {
    let texture_atlas = TextureAtlas::from_grid(
        asset_server.load("sprites/pacman_death.png"),
        Vec2::new(UNIT_SIZE as f32, UNIT_SIZE as f32),
        11,
        1,
        None,
        None
    );

    SpriteSheetBundle {
        texture_atlas: texture_atlases.add(texture_atlas),
        sprite: TextureAtlasSprite::new(0),
        ..default()
    }
}

pub fn spawn_pacman_death_animation(
    mut commands: Commands,
    mut query_pacman: Query<&UnitPosition, With<Pacman>>,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
    time: Res<Time>
) {
    if let Some(pac_pos) = query_pacman.iter_mut().next() {
        commands.spawn((
            PacmanDeathAnimation {
                start_animation_time: time.elapsed_seconds(),
                animation_time: 0.,
                played_sound: false,
            },
            pac_pos.clone(),
            UnitScale::square(0.95),
            load_pacman_death_sprite(asset_server, texture_atlases),
        ));
    }
}

pub fn despawn_pacman_death_animation(
    mut commands: Commands,
    mut query_pacman: Query<Entity, With<PacmanDeathAnimation>>,
) {
    if let Some(pacman_entity) = query_pacman.iter_mut().next() {
        commands.entity(pacman_entity).despawn();
    }
}

fn load_pacman_sprite(
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) -> SpriteSheetBundle {
    let texture_atlas = TextureAtlas::from_grid(
        asset_server.load("sprites/pacman.png"),
        Vec2::new(UNIT_SIZE as f32, UNIT_SIZE as f32),
        3,
        1,
        None,
        None
    );

    SpriteSheetBundle {
        texture_atlas: texture_atlases.add(texture_atlas),
        sprite: TextureAtlasSprite::new(0),
        ..default()
    }
}

pub fn spawn_pacman(
    mut commands: Commands,
    mut next_pacman_state: ResMut<NextState<PacmanState>>,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn((
        Pacman::new(),
        UnitPosition { x: PACMAN_START_X, y: PACMAN_START_Y },
        UnitScale::square(0.95),
        load_pacman_sprite(asset_server, texture_atlases),
    ));

    next_pacman_state.set(PacmanState::Normal);
}

pub fn despawn_pacman(
    mut commands: Commands,
    mut query_pacman: Query<Entity, With<Pacman>>,
) {
    if let Some(pacman_entity) = query_pacman.iter_mut().next() {
        commands.entity(pacman_entity).despawn();
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
        elapsed_startup: f32,
    ) {
        let elapsed = elapsed_startup - pacman.animation_time;
        if elapsed < 0.05 { return; }
        sprite.index = if sprite.index == 2 { 0 } else { sprite.index + 1 };

        transform.rotation = Quat::from_rotation_z(
            match pacman.current_direction {
                UnitDirection::Up => f32::to_radians(270.0),
                UnitDirection::Right => f32::to_radians(180.0),
                UnitDirection::Down => f32::to_radians(90.0),
                _ => f32::to_radians(0.)
        });
        
        pacman.animation_time = elapsed_startup;
    }

    if state.0 != GameState::Running { return; }

    if let Some((
        mut pacman,
        mut pos,
        mut sprite,
        mut transform,
    )) = query_pacman.iter_mut().next() {
        let pixel_speed = (time.delta_seconds() * PACMAN_SPEED )as i32;
        for _ in 0..pixel_speed {
            if unit_can_move_in_direction(&pos, pacman.next_direction) {
                pacman.current_direction = pacman.next_direction;
            } else if !unit_can_move_in_direction(&pos, pacman.current_direction) {
                break;
            }
            pos.move_in_direction(pacman.current_direction);
            animate(&mut pacman, &mut sprite, &mut transform, time.elapsed_seconds());
        }
    }
}

fn pacman_eats_dot(
    mut commands: Commands,
    mut game: ResMut<Game>,
    mut query_pacman: Query<(&mut Pacman, &UnitPosition)>,
    query_dot: Query<(Entity, &UnitPosition), With<Dot>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    if let Some((mut pac, pac_pos)) = query_pacman.iter_mut().next() {
        for (dot_entity, dot_pos) in query_dot.iter() {
            if units_collide(&pac_pos, UNIT_HITBOX_SIZE, &dot_pos, UNIT_HITBOX_SIZE) {
                // Add game points
                game.points += POINTS_DOT;
                
                // Despawn dot
                commands.entity(dot_entity).despawn();

                // Play eat sound
                pac.eaten_points += 1;
                audio.play(asset_server.load(
                    if pac.eaten_points % 2 == 0 { "sounds/eat2.ogg"} else { "sounds/eat.ogg" } ));
            }
        }
    }
}

pub fn pacman_eats_energizer(
    mut commands: Commands,
    mut game: ResMut<Game>,
    mut query_pacman: Query<(&mut Pacman, &UnitPosition)>,
    mut query_ghosts: Query<&mut Ghost>,
    query_energizer: Query<(Entity, &UnitPosition), With<Energizer>>,
    mut next_pacman_state: ResMut<NextState<PacmanState>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    time: Res<Time>,
) {
    if let Some((mut pac, pac_pos)) = query_pacman.iter_mut().next() {
        for (energizer_entity, energizer_pos) in query_energizer.iter() {
            if units_collide(&pac_pos, UNIT_HITBOX_SIZE, &energizer_pos, UNIT_HITBOX_SIZE) {
                // Add game points
                game.points += POINTS_ENERGIZER;

                // Despawn energizer
                commands.entity(energizer_entity).despawn();

                // Play eat sound
                pac.eaten_points += 1;
                audio.play(asset_server.load(
                    if pac.eaten_points % 2 == 0 { "sounds/eat2.ogg"} else { "sounds/eat.ogg" } ));
                
                // Mark pacman energized
                pac.eaten_ghosts = 0;
                pac.start_time_energized = time.elapsed_seconds();
                next_pacman_state.set(PacmanState::Energized);

                // Set elapsed time to immediately start playing sound
                game.elapsed_time_sound = time.elapsed_seconds() - SOUND_DURATION_AMBIENT_FRIGHT;

                // Mark ghosts as frightened
                for mut ghost in query_ghosts.iter_mut() {
                    ghost.is_frightened = true;
                }
            }
        }
    }
}

pub fn pacman_eats_ghost(
    mut game: ResMut<Game>,
    mut query_pacman: Query<(&mut Pacman, &UnitPosition)>,
    mut query_ghost: Query<(&mut Ghost, &mut UnitPosition), Without<Pacman>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    time: Res<Time>,
) {
    if let Some((mut pacman, pac_pos)) = query_pacman.iter_mut().next() {
        for (mut ghost, mut ghost_pos) in query_ghost.iter_mut() {
            if ghost.is_frightened && units_collide(&pac_pos, UNIT_HITBOX_SIZE, &ghost_pos, UNIT_HITBOX_SIZE) {
                // Play eat ghost sound
                audio.play(asset_server.load("sounds/eat_ghost.ogg"));
                
                // Reset ghost
                *ghost_pos = ghost.ghost_id.get_start_pos();
                ghost.reset(time.elapsed_seconds() + 5.);
                
                // Calculate points
                pacman.eaten_ghosts += 1;
                let mut points = POINTS_GHOST;                
                for _ in 0..pacman.eaten_ghosts {
                    points *= 2;
                }
                game.points += points;
            }
        }
    }
}

pub fn pacman_death_animation(
    mut next_pacman_state: ResMut<NextState<PacmanState>>,
    mut query_pacman: Query<(&mut PacmanDeathAnimation, &mut TextureAtlasSprite)>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    time: Res<Time>,
) {
    if let Some((mut pac, mut sprite)) = query_pacman.iter_mut().next() {
        // Play pacman death sound
        if !pac.played_sound {
            audio.play(asset_server.load("sounds/death.ogg"));
            pac.played_sound = true;
        }

        // Animate sprite every 0.15 seconds
        let elapsed_since_last_call = time.elapsed_seconds() - pac.animation_time;
        if elapsed_since_last_call > 0.15 && sprite.index < 10 {
            sprite.index += 1;
            pac.animation_time = time.elapsed_seconds();
        }
        
        // Change pacman state after 2 seconds to signal ready for respawn
        let elapsed_since_start = time.elapsed_seconds() - pac.start_animation_time;
        if elapsed_since_start >= 2.0 {
            next_pacman_state.set(PacmanState::Respawn);
        }
    }
}
