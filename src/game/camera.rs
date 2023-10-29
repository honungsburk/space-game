use super::player::components::Player;
use super::trauma::Trauma;
use super::{config::Flag, player::actions::PlayerAction};
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

pub struct CameraPlugin {
    pub is_debug: bool,
}

impl Default for CameraPlugin {
    fn default() -> Self {
        Self { is_debug: false }
    }
}

impl CameraPlugin {
    pub fn new(is_debug: bool) -> Self {
        Self { is_debug }
    }
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraPositionDebugFlag>()
            .init_resource::<CameraSetpointDebugFlag>()
            .add_systems(Startup, spawn)
            .add_systems(
                Update,
                (
                    update_smooth_camera,
                    update_shaky_camera,
                    // Debug
                    debug_camera_position.run_if(run_debug_camera_position),
                    debug_camera_setpoint.run_if(run_debug_camera_setpoint),
                ),
            );
    }
}

////////////////////////////////////////////////////////////////////////////////
// Components & Resources
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

////////////////////////////////////////////////////////////////////////////////
/// DEBUG
////////////////////////////////////////////////////////////////////////////////

#[derive(Resource)]
pub struct CameraPositionDebugFlag {
    pub flag: Flag,
}

impl Default for CameraPositionDebugFlag {
    fn default() -> Self {
        Self {
            flag: Flag::new(
                "Camera Position",
                "Display a crosshair where the player's camera is located",
                false,
            ),
        }
    }
}

#[derive(Resource, DerefMut, Deref)]
pub struct CameraSetpointDebugFlag {
    pub flag: Flag,
}

impl Default for CameraSetpointDebugFlag {
    fn default() -> Self {
        Self {
            flag: Flag::new(
                "Camera Setpoint",
                "Display a crosshair at the point to which the player's camera is moving towards",
                false,
            ),
        }
    }
}

fn run_debug_camera_position(camera_position_debug: Res<CameraPositionDebugFlag>) -> bool {
    camera_position_debug.flag.is_on()
}

fn run_debug_camera_setpoint(camera_setpoint_debug: Res<CameraSetpointDebugFlag>) -> bool {
    camera_setpoint_debug.flag.is_on()
}

fn debug_camera_position(mut gizmos: Gizmos, mut query: Query<&Transform, With<SmoothCamera>>) {
    if let Ok(transform) = query.get_single_mut() {
        // Crosshair for the camera's current position
        gizmo_crosshair(&mut gizmos, &transform.translation.xy(), Color::BLUE, 10.0);
    }
}

fn debug_camera_setpoint(mut gizmos: Gizmos, mut query: Query<&CameraPID, With<SmoothCamera>>) {
    if let Ok(pid) = query.get_single_mut() {
        // Crosshair for the camera's setpoint
        gizmo_crosshair(&mut gizmos, &pid.get_setpoint(), Color::GREEN, 10.0);
    }
}

fn gizmo_crosshair(gizmos: &mut Gizmos, position: &Vec2, color: Color, size: f32) {
    let x = position.x;
    let y = position.y;

    gizmos.line(
        Vec3::new(x - size, y, 0.0),
        Vec3::new(x + size, y, 0.0),
        color,
    );
    gizmos.line(
        Vec3::new(x, y - size, 0.0),
        Vec3::new(x, y + size, 0.0),
        color,
    );
}
