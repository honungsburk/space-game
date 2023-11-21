mod fps_counter;
mod health_bar;
mod score_tracker;

use bevy::prelude::*;
use bevy_progressbar::ProgressBarMaterial;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            fps_counter::FPSCounterPlugin,
            health_bar::HealthBarPlugin,
            score_tracker::ScoreTrackerPlugin,
        ))
        .add_systems(Startup, spawn_hud);
    }
}

// Components

#[derive(Component)]
struct HUD {}

fn spawn_hud(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ProgressBarMaterial>>,
) {
    build_hud(&mut commands, &asset_server, &mut materials);
}

const BACKGROUND_COLOR: Color = Color::rgba(0.25, 0.25, 0.25, 0.5);

fn build_hud(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    materials: &mut ResMut<Assets<ProgressBarMaterial>>,
) {
    let health_bar_id = health_bar::build(commands, asset_server);
    let fps_counter_id = fps_counter::build(commands, asset_server);
    let score_tracker_id = score_tracker::build(commands, asset_server, materials);

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

    hud_entity.push_children(&[health_bar_id, fps_counter_id, score_tracker_id]);
}

fn despawn_hud(mut commands: Commands, hud_query: Query<Entity, With<HUD>>) {
    for entity in hud_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
