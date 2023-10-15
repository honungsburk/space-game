use crate::game::{player::components::Player, score::GameScore, vitality::Health};
use bevy::prelude::*;
use bevy_progressbar::{ProgressBarBundle, ProgressBarSections};

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy_progressbar::ProgressBarPlugin)
            // OnEnter Systems
            .add_systems(Startup, spawn_hud)
            // Systems
            .add_systems(Update, (update_score_tracker, update_health_bar));
    }
}

// Components

#[derive(Component)]
struct HUD {}

fn spawn_hud(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
) {
    build_hud(&mut commands, &asset_server, &mut images);
}

const BACKGROUND_COLOR: Color = Color::rgba(0.25, 0.25, 0.25, 0.5);

fn build_hud(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    images: &mut ResMut<Assets<Image>>,
) {
    let health_bar_id = build_health_bar(commands, asset_server);
    let score_tracker_id = build_score_tracker(commands, asset_server, images);

    let mut hud_entity = commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Start,
                // top: Val::Px(20.0),
                width: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            border_color: Color::rgb(0.5, 0.5, 0.5).into(),
            ..default()
        },
        HUD {},
    ));

    hud_entity.push_children(&[health_bar_id, score_tracker_id]);
}

fn despawn_hud(mut commands: Commands, hud_query: Query<Entity, With<HUD>>) {
    for entity in hud_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

//////////////////////////////////////////////////////////////////////////////
/// Score
//////////////////////////////////////////////////////////////////////////////

#[derive(Component)]
struct MultiplierScore {}

#[derive(Component)]
struct Multiplier {}

#[derive(Component)]
struct MultiplierTimer {}

const MULTIPLIER_TIME_AMOUNT: u32 = 1000;

fn build_multiplier_timer(commands: &mut Commands, images: &mut ResMut<Assets<Image>>) -> Entity {
    // Multiplier Timer
    let multiplier_timer_id = commands
        .spawn(
            ProgressBarBundle::new(MULTIPLIER_TIME_AMOUNT, 250, 5, images)
                .add_section(MULTIPLIER_TIME_AMOUNT / 2, Color::DARK_GRAY)
                .add_section(MULTIPLIER_TIME_AMOUNT / 2, Color::WHITE),
        )
        .insert(MultiplierTimer {})
        .id();

    multiplier_timer_id
}

#[derive(Component)]
struct LockedScore {}

fn build_score_tracker(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    images: &mut ResMut<Assets<Image>>,
) -> Entity {
    // Multiplier Tracker
    let multiplier_score_id = commands
        .spawn((
            MultiplierScore {},
            TextBundle {
                style: Style { ..default() },
                text: Text {
                    sections: vec![TextSection::new(
                        "0",
                        TextStyle {
                            // font: font_future(asset_server),
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
            Multiplier {},
            TextBundle {
                style: Style { ..default() },
                text: Text {
                    sections: vec![TextSection::new(
                        "1",
                        TextStyle {
                            // font: font_future(asset_server),
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

    let multiplier_timer_id = build_multiplier_timer(commands, images);

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
                            // font: font_future(asset_server),
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

fn update_multiplier_score(
    mut text_query: Query<&mut Text, With<MultiplierScore>>,
    game_score: Res<GameScore>,
) {
    if game_score.is_changed() {
        for mut text in text_query.iter_mut() {
            text.sections[0].value = format!("{}", game_score.multiplier().to_string());
        }
    }
}

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
    mut multiplier_timer_query: Query<&mut ProgressBarSections, With<MultiplierTimer>>,
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
        for mut sections in multiplier_timer_query.iter_mut() {
            if let Some(precent_left) = game_score.multiplier_time_percent_left() {
                let left = (precent_left * (MULTIPLIER_TIME_AMOUNT as f32)) as u32;
                sections.0 = vec![
                    (MULTIPLIER_TIME_AMOUNT - left, Color::DARK_GRAY),
                    (left, Color::WHITE),
                ];
            } else {
                sections.0 = vec![(MULTIPLIER_TIME_AMOUNT, Color::DARK_GRAY)];
            }
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
/// Health Bar
//////////////////////////////////////////////////////////////////////////////

#[derive(Component)]
struct HealthBar {}

fn build_health_bar(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    // Score Text
    commands
        .spawn((
            HealthBar {},
            TextBundle {
                style: Style { ..default() },
                text: Text {
                    sections: vec![TextSection::new(
                        "100",
                        TextStyle {
                            // font: font_future(asset_server),
                            font_size: 48.0,
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
        .id()
}

fn update_health_bar(
    mut text_query: Query<&mut Text, With<HealthBar>>,
    player_health_query: Query<&Health, With<Player>>,
) {
    if let Ok(player_health) = player_health_query.get_single() {
        for mut text in text_query.iter_mut() {
            text.sections[0].value = format!("{}", player_health.current().to_string());
            if player_health.current() < 33 {
                text.sections[0].style.color = Color::RED;
            } else if player_health.current() < 66 {
                text.sections[0].style.color = Color::ORANGE;
            } else {
                text.sections[0].style.color = Color::WHITE;
            }
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
/// Styles
//////////////////////////////////////////////////////////////////////////////

/// TODO: Font isn't loaded properly
fn font_future(asset_server: &Res<AssetServer>) -> Handle<Font> {
    asset_server.load("fonts/kenvector_future.tff")
}

/// TODO: Font isn't loaded properly
fn font_future_thin(asset_server: &Res<AssetServer>) -> Handle<Font> {
    asset_server.load("fonts/kenvector_future_thin.tff")
}
