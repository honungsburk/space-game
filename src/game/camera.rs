use super::player::actions::PlayerAction;
use super::player::components::Player;
use super::trauma::Trauma;
use crate::misc::control::PID;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::Velocity;
use leafwing_input_manager::prelude::ActionState;
use noise::{Fbm, NoiseFn, Perlin, Seedable};

////////////////////////////////////////////////////////////////////////////////
// Plugin
////////////////////////////////////////////////////////////////////////////////

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn)
            .add_systems(Update, (update_smooth_camera, update_shaky_camera));
    }
}

////////////////////////////////////////////////////////////////////////////////
// Components
////////////////////////////////////////////////////////////////////////////////

// This camera is smooth and follows the player + uses look ahead
#[derive(Component)]
pub struct SmoothCamera;

// This camera is what the player sees. On every frame, it is moved to the player camera's position + add screen shake if there is any
#[derive(Component)]
pub struct ShakyCamera {
    pub x_perlin: Fbm<Perlin>,
    pub y_perlin: Fbm<Perlin>,
    pub angel_perlin: Fbm<Perlin>,
}

impl Default for ShakyCamera {
    fn default() -> Self {
        let fbm = Fbm::<Perlin>::default();
        // fbm.frequency = 10.0;

        Self {
            x_perlin: fbm.clone().set_seed(124135),
            y_perlin: fbm.clone().set_seed(123),
            angel_perlin: fbm.clone().set_seed(43212),
        }
    }
}

// Camera PID controller
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
    let transform = Transform::from_xyz(x, y, 0.0);

    // Smooth camera
    commands
        .spawn(TransformBundle::from(transform))
        .insert(SmoothCamera)
        .insert(CameraPID::from_xy(x, y));

    // Shaky camera (What the player sees)
    commands
        .spawn(Camera2dBundle {
            transform: transform,
            ..default()
        })
        .insert(ShakyCamera::default());
}

// pub fn debug_camera_position(
//     mut commands: Commands,
//     mut query: Query<(&mut Transform, &mut CameraPID), (Without<Player>, With<Camera>)>,
// ) {
//     if let Ok((mut transform, mut pid)) = query.get_single_mut() {
//         // Create a debug entity to show the camera's position
//         commands.spawn(bundle)
//     }
// }

pub fn update_smooth_camera(
    time: Res<Time>,
    mut camera_query: Query<
        (&mut Transform, &mut CameraPID),
        (Without<Player>, With<SmoothCamera>),
    >,
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

            camera_pid.set_setpoint(desired_camera_placement);

            // Determine desired camera zoom level

            // Move camera

            let control_signal =
                camera_pid.update(camera_transform.translation.xy(), time.delta_seconds());

            camera_transform.translation += Vec3::new(control_signal.x, control_signal.y, 0.0);
        }
    }
}

const MAX_SHAKY_ANGEL: f32 = 0.05;
const MAX_SHAKY_OFFSET: f32 = 4.0;

fn update_shaky_camera(
    time: Res<Time>,
    mut shaky_camera_query: Query<
        (&mut Transform, &mut ShakyCamera),
        (Without<Player>, Without<SmoothCamera>),
    >,
    smooth_camera_query: Query<
        &Transform,
        (Without<Player>, With<SmoothCamera>, Without<ShakyCamera>),
    >,
    player_query: Query<
        Option<&Trauma>,
        (With<Player>, Without<SmoothCamera>, Without<ShakyCamera>),
    >,
) {
    if let (
        Ok(player_trauma_op),
        Ok(smooth_camera_transform),
        Ok((mut shaky_camera_transform, shaky_camera)),
    ) = (
        player_query.get_single(),
        smooth_camera_query.get_single(),
        shaky_camera_query.get_single_mut(),
    ) {
        if let Some(player_trauma) = player_trauma_op {
            let trauma = player_trauma.get_trauma();
            let shake = trauma.powi(2);

            // let rng = &mut rand::thread_rng();
            // let uniform: Uniform<f32> = Uniform::new(-1.0, 1.0);

            // If you slow down the game, the camera will shake smoothly
            let sample_point = [time.elapsed_seconds_f64(), time.elapsed_seconds_f64()];

            // println!(
            //     "Sample point: {:?}",
            //     (shaky_camera.angel_perlin.get(sample_point) as f32)
            // );

            let random_angel =
                MAX_SHAKY_ANGEL * shake * (shaky_camera.angel_perlin.get(sample_point) as f32);
            let random_offset_x =
                MAX_SHAKY_OFFSET * shake * (shaky_camera.x_perlin.get(sample_point) as f32);
            let random_offset_y =
                MAX_SHAKY_OFFSET * shake * (shaky_camera.y_perlin.get(sample_point) as f32);

            // let random_angel = MAX_SHAKY_ANGEL * shake * uniform.sample(rng);
            // let random_offset_x = MAX_SHAKY_OFFSET * shake * uniform.sample(rng);
            // let random_offset_y = MAX_SHAKY_OFFSET * shake * uniform.sample(rng);

            shaky_camera_transform.translation = smooth_camera_transform.translation
                + Vec3::new(random_offset_x, random_offset_y, 0.0);

            shaky_camera_transform.rotation = smooth_camera_transform
                .rotation
                .mul_quat(Quat::from_rotation_z(random_angel));
            shaky_camera_transform.scale = smooth_camera_transform.scale;
        } else {
            shaky_camera_transform.translation = smooth_camera_transform.translation;
            shaky_camera_transform.rotation = smooth_camera_transform.rotation;
            shaky_camera_transform.scale = smooth_camera_transform.scale;
        }
    }
}
