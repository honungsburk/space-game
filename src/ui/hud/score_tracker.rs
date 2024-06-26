use crate::{game::score::GameScore, ui::assets::GameFonts};
use bevy::prelude::*;
use bevy_progressbar::{ProgressBar, ProgressBarBundle, ProgressBarMaterial};

////////////////////////////////////////////////////////////////////////////////
// Plugin
////////////////////////////////////////////////////////////////////////////////

pub struct ScoreTrackerPlugin;

impl Plugin for ScoreTrackerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy_progressbar::ProgressBarPlugin)
            .add_systems(Update, update_score_tracker);
    }
}

////////////////////////////////////////////////////////////////////////////////
// Components
////////////////////////////////////////////////////////////////////////////////

#[derive(Component)]
struct MultiplierScore;

#[derive(Component)]
struct Multiplier;

#[derive(Component)]
struct MultiplierTimer;

#[derive(Component)]
struct LockedScore;

////////////////////////////////////////////////////////////////////////////////
// Builders
////////////////////////////////////////////////////////////////////////////////

const MULTIPLIER_TIME_AMOUNT: u32 = 1000;

fn build_multiplier_timer(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<ProgressBarMaterial>>,
) -> Entity {
    let mut bar = ProgressBar::new(vec![
        (MULTIPLIER_TIME_AMOUNT / 2, Color::DARK_GRAY),
        (MULTIPLIER_TIME_AMOUNT / 2, Color::WHITE),
    ]);

    bar.set_progress(1.0);

    let style = Style {
        width: Val::Px(400.0),
        height: Val::Px(4.0),
        right: Val::Px(0.0),
        ..default()
    };

    // Multiplier Timer
    let multiplier_timer_id = commands
        .spawn(MultiplierTimer)
        .insert(ProgressBarBundle::new(style, bar, materials))
        .id();

    multiplier_timer_id
}

pub fn build(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    materials: &mut ResMut<Assets<ProgressBarMaterial>>,
) -> Entity {
    // Multiplier Tracker
    let multiplier_score_id = commands
        .spawn((
            MultiplierScore,
            TextBundle {
                style: Style { ..default() },
                text: Text {
                    sections: vec![TextSection::new(
                        "0",
                        TextStyle {
                            font: asset_server.font_future(),
                            font_size: 48.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    )],

                    ..default()
                },
                ..default()
            },
        ))
        .id();

    let x_id = commands
        .spawn(TextBundle {
            style: Style { ..default() },
            text: Text {
                sections: vec![TextSection::new(
                    "x",
                    TextStyle {
                        font_size: 48.0,
                        color: Color::WHITE,
                        ..default()
                    },
                )],

                ..default()
            },
            ..default()
        })
        .id();

    let multiplier_id = commands
        .spawn((
            Multiplier,
            TextBundle {
                style: Style { ..default() },
                text: Text {
                    sections: vec![TextSection::new(
                        "0",
                        TextStyle {
                            font: asset_server.font_future(),
                            font_size: 68.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    )],

                    ..default()
                },
                ..default()
            },
        ))
        .id();

    let multiplier_display = commands
        .spawn((NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                // justify_content: JustifyContent::Start,
                align_items: AlignItems::Center,
                column_gap: Val::Px(10.0),
                ..default()
            },
            border_color: Color::rgb(0.5, 0.5, 0.5).into(),
            ..default()
        },))
        .push_children(&[multiplier_score_id, x_id, multiplier_id])
        .id();

    // Multiplier Timer

    let multiplier_timer_id = build_multiplier_timer(commands, materials);

    // Locked Score

    let locked_score_id = commands
        .spawn((
            LockedScore {},
            TextBundle {
                style: Style { ..default() },
                text: Text {
                    sections: vec![TextSection::new(
                        "0",
                        TextStyle {
                            font: asset_server.font_future(),
                            font_size: 32.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    )],
                    alignment: TextAlignment::Center,
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    let mut score_tracker = commands.spawn(NodeBundle {
        style: Style {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Start,
            align_items: AlignItems::End,
            column_gap: Val::Px(10.0),
            ..default()
        },
        border_color: Color::rgb(0.5, 0.5, 0.5).into(),
        ..default()
    });

    score_tracker.push_children(&[multiplier_timer_id, multiplier_display, locked_score_id]);

    score_tracker.id()
}

////////////////////////////////////////////////////////////////////////////////
// Systems
////////////////////////////////////////////////////////////////////////////////

fn update_score_tracker(
    mut multiplier_score_query: Query<
        &mut Text,
        (
            With<MultiplierScore>,
            Without<Multiplier>,
            Without<LockedScore>,
        ),
    >,
    mut multiplier_query: Query<
        &mut Text,
        (
            Without<MultiplierScore>,
            With<Multiplier>,
            Without<LockedScore>,
        ),
    >,
    mut locked_score_query: Query<
        &mut Text,
        (
            Without<MultiplierScore>,
            Without<Multiplier>,
            With<LockedScore>,
        ),
    >,
    mut multiplier_timer_query: Query<&mut ProgressBar, With<MultiplierTimer>>,
    game_score: Res<GameScore>,
) {
    if game_score.is_changed() {
        for mut text in multiplier_score_query.iter_mut() {
            text.sections[0].value = game_score.current_multiplier_score().to_string();
        }
        for mut text in multiplier_query.iter_mut() {
            text.sections[0].value = game_score.multiplier().to_string();
        }
        for mut text in locked_score_query.iter_mut() {
            text.sections[0].value = game_score.locked_in_score().to_string();
        }
        for mut multiplier_progressbar in multiplier_timer_query.iter_mut() {
            if let Some(precent_left) = game_score.multiplier_time_percent_left() {
                let left = (precent_left * (MULTIPLIER_TIME_AMOUNT as f32)) as u32;

                multiplier_progressbar.sections = vec![
                    (MULTIPLIER_TIME_AMOUNT - left, Color::DARK_GRAY),
                    (left, Color::WHITE),
                ];
            } else {
                multiplier_progressbar.sections = vec![(MULTIPLIER_TIME_AMOUNT, Color::DARK_GRAY)];
            }
        }
    }
}
