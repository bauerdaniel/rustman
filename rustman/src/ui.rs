//
// Daniel Bauer (bauerda@pm.me)
//

use bevy::prelude::*;

use super::states::*;
use super::game::*;

pub const UI_HEIGHT: u32 = 100;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup_ui)
            .add_systems((
                ui_update_status,
                ui_update_round_text,
                ui_update_points_text,
                ui_update_life_text,
            ))
        ;
    }
}

#[derive(Component)]
pub struct StatusText;

#[derive(Component)]
pub struct PointsText;

#[derive(Component)]
pub struct RoundText;

#[derive(Component)]
pub struct LifesText;

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Status Text
    commands.spawn((
        TextBundle::from_sections([
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 30.0,
                color: Color::YELLOW,
            }),
        ])
        .with_text_alignment(TextAlignment::Right)
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(20.0),
                right: Val::Px(25.0),
                ..default()
            },
            ..default()
        }),
        StatusText,
    ));

    // Round Text
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Round: ",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 15.0,
                    color: Color::GRAY,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: 15.0,
                color: Color::GRAY,
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(20.0),
                left: Val::Px(25.0),
                ..default()
            },
            ..default()
        }),
        RoundText,
    ));

    // Points Text
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Points: ",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 15.0,
                    color: Color::GRAY,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: 15.0,
                color: Color::GRAY,
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(40.0),
                left: Val::Px(25.0),
                ..default()
            },
            ..default()
        }),
        PointsText,
    ));

    // Lifes Text
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Lifes: ",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 15.0,
                    color: Color::GRAY,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: 15.0,
                color: Color::GRAY,
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Px(40.0),
                left: Val::Px(25.0),
                ..default()
            },
            ..default()
        }),
        LifesText,
    ));
}

fn ui_update_status(
    state: Res<State<GameState>>,
    mut query: Query<&mut Text, With<StatusText>>,
) {
    if let Some(mut text) = query.iter_mut().next() {
        let status = match state.0 {
            GameState::Ready => "Ready!",
            GameState::Paused => "Paused!",
            GameState::GameOver => "Game Over!",
            _ => "",
        };
        text.sections[0].value = status.to_string();
    }
}

fn ui_update_round_text(
    game: Res<Game>,
    mut query: Query<&mut Text, With<RoundText>>
) {
    if let Some(mut text) = query.iter_mut().next() {
        text.sections[1].value = format!("{}", game.round);
    }
}

fn ui_update_points_text(
    game: Res<Game>,
    mut query: Query<&mut Text, With<PointsText>>
) {
    if let Some(mut text) = query.iter_mut().next() {
        text.sections[1].value = format!("{}", game.points);
    }
}

fn ui_update_life_text(
    game: Res<Game>,
    mut query: Query<&mut Text, With<LifesText>>
) {
    if let Some(mut text) = query.iter_mut().next() {
        text.sections[1].value = format!("{}", game.lifes);
    }
}
