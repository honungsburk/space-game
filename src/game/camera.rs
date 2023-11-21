use super::debug::{self, CameraPositionDebugFlagLabel, CameraSetpointDebugFlagLabel};
use super::input::InputAction;
use super::player::PlayerShipAction;
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
        app.init_resource::<ScreenBounds>().add_systems(
            Update,
            (
                update_smooth_camera,
                update_shaky_camera.after(update_smooth_camera),
                update_screen_bounds,
                // Debug
                debug_camera_position.run_if(debug::flag_is_on::<CameraPositionDebugFlagLabel>),
                debug_camera_setpoint.run_if(debug::flag_is_on::<CameraSetpointDebugFlagLabel>),
            ),
        );
    }
}

////////////////////////////////////////////////////////////////////////////////
// Components & Resources
////////////////////////////////////////////////////////////////////////////////

#[derive(Component)]
pub struct CameraTargetLabel;

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

#[derive(Resource)]
pub struct ScreenBounds {
    center: Vec2,
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
}

impl Default for ScreenBounds {
    fn default() -> Self {
        Self {
            center: Vec2::ZERO,
            left: 0.0,
            right: 0.0,
            top: 0.0,
            bottom: 0.0,
        }
    }
}

impl ScreenBounds {
    fn new(width: f32, height: f32, position: Vec2) -> Self {
        let left = position.x - width / 2.0;
        let right = position.x + width / 2.0;
        let top = position.y + height / 2.0;
        let bottom = position.y - height / 2.0;

        Self {
            center: position,
            left,
            right,
            top,
            bottom,
        }
    }

    fn update(&mut self, width: f32, height: f32, position: Vec2) -> &mut Self {
        self.left = position.x - width / 2.0;
        self.right = position.x + width / 2.0;
        self.top = position.y + height / 2.0;
        self.bottom = position.y - height / 2.0;
        self.center = position;

        self
    }

    pub fn contains(&self, position: Vec2) -> bool {
        position.x > self.left
            && position.x < self.right
            && position.y > self.bottom
            && position.y < self.top
    }
}

////////////////////////////////////////////////////////////////////////////////
// Systems
////////////////////////////////////////////////////////////////////////////////

fn update_screen_bounds(
    mut screen_bounds: ResMut<ScreenBounds>,
    window_query: Query<&Window, (With<PrimaryWindow>, Without<ShakyCamera>)>,
    camera_query: Query<&Transform, (Without<PrimaryWindow>, With<ShakyCamera>)>,
) {
    if let Ok(window) = window_query.get_single() {
        if let Ok(transform) = camera_query.get_single() {
            let width = window.width();
            let height = window.height();
            let position = transform.translation.xy();

            screen_bounds.update(width, height, position);
        }
    }
}

pub fn spawn(mut commands: Commands) {
    let transform = Transform::from_xyz(0.0, 0.0, 0.0);

    // Smooth camera
    commands
        .spawn(TransformBundle::from(transform))
        .insert(SmoothCamera)
        .insert(CameraPID::from_xy(0.0, 0.0));

    // Shaky camera (What the player sees)
    commands
        .spawn(Camera2dBundle {
            transform: transform,
            ..default()
        })
        .insert(ShakyCamera::default());
}

pub fn despawn(
    mut commands: Commands,
    query: Query<Entity, Or<(With<SmoothCamera>, With<ShakyCamera>)>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn update_smooth_camera(
    time: Res<Time>,
    mut camera_query: Query<
        (&mut Transform, &mut CameraPID),
        (Without<CameraTargetLabel>, With<SmoothCamera>),
    >,
    input_query: Query<&ActionState<InputAction>, Without<CameraTargetLabel>>,
    target_query: Query<(&Transform, &Velocity), (With<CameraTargetLabel>, Without<Camera>)>,
) {
    if time.delta_seconds() == 0.0 {
        return;
    }

    if let (Ok((target_transform, target_velocity)), Ok(input_action)) =
        (target_query.get_single(), input_query.get_single())
    {
        if let Ok((mut camera_transform, mut camera_pid)) = camera_query.get_single_mut() {
            // Determine desired camera placement
            let player_rotation = if let Some(axis_data) = input_action
                .clamped_axis_pair(InputAction::PlayerShip(PlayerShipAction::RotateShip))
            {
                100.0 * axis_data.xy()
            } else {
                target_transform.rotation.mul_vec3(Vec3::Y).xy().normalize() * 100.0
            };

            let desired_camera_placement = target_transform.translation.xy()
                + (player_rotation + target_velocity.linvel).clamp_length(0.0, 200.0);

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
        (Without<CameraTargetLabel>, Without<SmoothCamera>),
    >,
    smooth_camera_query: Query<
        &Transform,
        (
            Without<CameraTargetLabel>,
            With<SmoothCamera>,
            Without<ShakyCamera>,
        ),
    >,
    target_query: Query<
        Option<&Trauma>,
        (
            With<CameraTargetLabel>,
            Without<SmoothCamera>,
            Without<ShakyCamera>,
        ),
    >,
) {
    if let (
        Ok(player_trauma_op),
        Ok(smooth_camera_transform),
        Ok((mut shaky_camera_transform, shaky_camera)),
    ) = (
        target_query.get_single(),
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
