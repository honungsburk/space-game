use crate::misc::control::PID;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::Velocity;
use leafwing_input_manager::prelude::ActionState;

use super::player::actions::PlayerAction;
use super::player::components::Player;

////////////////////////////////////////////////////////////////////////////////
// Plugin
////////////////////////////////////////////////////////////////////////////////

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn)
            .add_systems(Update, update_camera);
    }
}

////////////////////////////////////////////////////////////////////////////////
// Components
////////////////////////////////////////////////////////////////////////////////

#[derive(Component)]
pub struct CameraPID {
    pub x_pid: PID,
    pub y_pid: PID,
}

impl CameraPID {
    pub fn from_xy(x: f32, y: f32) -> Self {
        Self {
            x_pid: PID::basic(0.05, 0.1, 0.0, x),
            y_pid: PID::basic(0.05, 0.1, 0.0, y),
        }
    }

    pub fn new(xy: Vec2) -> Self {
        Self::from_xy(xy.x, xy.y)
    }

    pub fn update(&mut self, measured_value: Vec2, dt: f32) -> Vec2 {
        Vec2::new(
            self.x_pid.update(measured_value.x, dt),
            self.y_pid.update(measured_value.y, dt),
        )
    }

    pub fn set_setpoint(&mut self, setpoint: Vec2) {
        self.x_pid.set_setpoint(setpoint.x);
        self.y_pid.set_setpoint(setpoint.y);
    }

    pub fn get_setpoint(&self) -> Vec2 {
        Vec2::new(self.x_pid.get_setpoint(), self.y_pid.get_setpoint())
    }
}

////////////////////////////////////////////////////////////////////////////////
// Systems
////////////////////////////////////////////////////////////////////////////////

pub fn spawn(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    let x = window.width() / 2.0;
    let y = window.height() / 2.0;

    commands
        .spawn(Camera2dBundle {
            transform: Transform::from_xyz(x, y, 0.0),
            ..default()
        })
        .insert(CameraPID::from_xy(x, y));
}

pub fn update_camera(
    time: Res<Time>,
    mut camera_query: Query<(&mut Transform, &mut CameraPID), (Without<Player>, With<Camera>)>,
    player_query: Query<
        (&Transform, &Velocity, &ActionState<PlayerAction>),
        (With<Player>, Without<Camera>),
    >,
) {
    if time.delta_seconds() == 0.0 {
        return;
    }

    if let Ok((player_transform, player_velocity, player_action)) = player_query.get_single() {
        if let Ok((mut camera_transform, mut camera_pid)) = camera_query.get_single_mut() {
            // Determine desired camera placement

            let player_rotation = if let Some(axis_data) =
                player_action.clamped_axis_pair(PlayerAction::RotateShip)
            {
                100.0 * axis_data.xy()
            } else {
                player_transform.rotation.mul_vec3(Vec3::Y).xy().normalize() * 100.0
            };

            let desired_camera_placement = player_transform.translation.xy()
                + (player_rotation + player_velocity.linvel).clamp_length(0.0, 200.0);

            let current_desired_location = camera_pid.get_setpoint();

            // How much faster/slower the camera can move than the player
            let max_relative_camera_speed = 20.0;
            let max_camera_speed = player_velocity.linvel.length() + max_relative_camera_speed;

            // let max_camera_velocity =
            //     (player_velocity.linvel.normalize_or_zero() + max_camera_speed).min(rhs);

            camera_pid.set_setpoint(desired_camera_placement);

            // Determine desired camera zoom level

            // Move camera

            let control_signal =
                camera_pid.update(camera_transform.translation.xy(), time.delta_seconds());

            camera_transform.translation += Vec3::new(control_signal.x, control_signal.y, 0.0);
        }
    }
}
