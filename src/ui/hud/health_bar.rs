use crate::{
    game::{player::components::Player, vitality::Health},
    ui::assets::GameFonts,
};
use bevy::prelude::*;

////////////////////////////////////////////////////////////////////////////////
// Plugin
////////////////////////////////////////////////////////////////////////////////
pub struct HealthBarPlugin;

impl Plugin for HealthBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_health_bar);
    }
}

////////////////////////////////////////////////////////////////////////////////
// Components
////////////////////////////////////////////////////////////////////////////////

#[derive(Component)]
struct HealthBar;

////////////////////////////////////////////////////////////////////////////////
// Builders
////////////////////////////////////////////////////////////////////////////////

pub fn build(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    // Score Text
    commands
        .spawn((
            HealthBar,
            TextBundle {
                style: Style { ..default() },
                text: Text {
                    sections: vec![TextSection::new(
                        "100",
                        TextStyle {
                            font: asset_server.font_future(),
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

////////////////////////////////////////////////////////////////////////////////
// Systems
////////////////////////////////////////////////////////////////////////////////

fn update_health_bar(
    mut text_query: Query<&mut Text, With<HealthBar>>,
    player_health_query: Query<&Health, With<Player>>,
) {
    if let Ok(player_health) = player_health_query.get_single() {
        for mut text in text_query.iter_mut() {
            text.sections[0].value = player_health.current().to_string();
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
