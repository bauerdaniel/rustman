//
// Daniel Bauer (bauerda@pm.me)
//

use bevy::prelude::*;
use super::game::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_ui)
            .add_system(update_ui);
    }
}

#[derive(Component)]
pub struct PointsText;

#[derive(Component)]
pub struct RoundText;

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Game State
    commands.insert_resource(Game {
        round: 1,
        points: 0
    });

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
}

pub fn update_ui(
    game: Res<Game>,
    mut q_round: Query<&mut Text, (With<RoundText>, Without<PointsText>)>,
    mut q_points: Query<&mut Text, (With<PointsText>, Without<RoundText>)>,
) {
    for mut text in &mut q_round {
        let round = game.round;
        text.sections[1].value = format!("{round}");
    }

    for mut text in &mut q_points {
        let points = game.points;
        text.sections[1].value = format!("{points}");
    }
}