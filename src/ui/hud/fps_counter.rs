use crate::{game::config::Flag, ui::assets::GameFonts};
use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

////////////////////////////////////////////////////////////////////////////////
// Plugin
////////////////////////////////////////////////////////////////////////////////

pub struct FPSCounterPlugin;

impl Plugin for FPSCounterPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FPSCounterDebugFlag {
            flag: Flag::new("FPS Counter", "Show fps count in the hud", true),
        })
        .insert_resource(UpdateFPSTimer(Timer::from_seconds(
            0.5,
            TimerMode::Repeating,
        )))
        .add_systems(
            Update,
            (
                update_fps_counter_timer,
                update_fps_counter_visability,
                update_fps_counter.run_if(should_show_fps_counter),
            ),
        );
    }
}

////////////////////////////////////////////////////////////////////////////////
// Components & Resources
////////////////////////////////////////////////////////////////////////////////

#[derive(Component, Debug)]
struct FPSCounter;

#[derive(Resource, DerefMut, Deref, Debug)]
pub struct FPSCounterDebugFlag {
    pub flag: Flag,
}
#[derive(Resource, DerefMut, Deref, Debug)]
struct UpdateFPSTimer(Timer);

////////////////////////////////////////////////////////////////////////////////
// Builders
////////////////////////////////////////////////////////////////////////////////

pub fn build(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    // Multiplier Tracker
    let fps_counter_id = commands
        .spawn((
            FPSCounter,
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

    return fps_counter_id;
}

////////////////////////////////////////////////////////////////////////////////
// Systems
////////////////////////////////////////////////////////////////////////////////

pub fn should_show_fps_counter(fps_counter_debug_flag: Res<FPSCounterDebugFlag>) -> bool {
    fps_counter_debug_flag.flag.is_on()
}

fn update_fps_counter_visability(
    fps_counter_debug_flag: Res<FPSCounterDebugFlag>,
    mut fps_counter_visibility_query: Query<&mut Visibility, With<FPSCounter>>,
) {
    if fps_counter_debug_flag.is_changed() {
        if let Ok(mut visibility) = fps_counter_visibility_query.get_single_mut() {
            *visibility = if fps_counter_debug_flag.flag.is_on() {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            };
        }
    }
}

fn update_fps_counter_timer(time: Res<Time>, mut update_fps_timer: ResMut<UpdateFPSTimer>) {
    update_fps_timer.0.tick(time.delta());
}

fn update_fps_counter(
    fps_timer: Res<UpdateFPSTimer>,
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<(&mut Text, &FPSCounter)>,
) {
    if fps_timer.finished() {
        for (mut text, _) in query.iter_mut() {
            if let Some(fps) = diagnostics.get_measurement(FrameTimeDiagnosticsPlugin::FPS) {
                text.sections[0].value = format!("{:.0}", fps.value);
            }
        }
    }
}
