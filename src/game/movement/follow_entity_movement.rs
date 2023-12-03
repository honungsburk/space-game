use crate::game::debug::{self, CameraPositionDebugFlagLabel, CameraSetpointDebugFlagLabel};
use crate::misc::gizmos;
use crate::{
    game::player::PlayerShipAction,
    misc::control::{PID, PID2D},
};
use bevy::prelude::*;
use bevy_rapier2d::dynamics::Velocity;
use leafwing_input_manager::action_state::ActionState;
use serde::de;
////////////////////////////////////////////////////////////////////////////////
/// Plugin
////////////////////////////////////////////////////////////////////////////////

pub struct FollowEntityMovementPlugin;

impl Plugin for FollowEntityMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update,
                debug_position.run_if(debug::flag_is_on::<CameraPositionDebugFlagLabel>),
                debug_setpoint.run_if(debug::flag_is_on::<CameraSetpointDebugFlagLabel>),
            ),
        );
    }
}

////////////////////////////////////////////////////////////////////////////////
/// Bundle
////////////////////////////////////////////////////////////////////////////////

#[derive(Bundle)]
pub struct FollowEntityMovementBundle {
    pub movement: FollowEntityMovement,
    pub local: Transform,
    pub global: GlobalTransform,
}

impl Default for FollowEntityMovementBundle {
    fn default() -> Self {
        Self {
            movement: FollowEntityMovement::default(),
            local: Transform::default(),
            global: GlobalTransform::default(),
        }
    }
}

impl FollowEntityMovementBundle {
    pub fn new(target: Option<Entity>, pid: PID2D) -> Self {
        Self {
            movement: FollowEntityMovement::new(target, pid),
            ..default()
        }
    }

    pub fn basic(target: Entity) -> Self {
        Self {
            movement: FollowEntityMovement::basic(target),
            ..default()
        }
    }

    pub fn smooth(target: Entity) -> Self {
        Self {
            movement: FollowEntityMovement::smooth(target),
            ..default()
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
/// Components
////////////////////////////////////////////////////////////////////////////////

#[derive(Component)]
pub struct FollowEntityMovement {
    pub target: Option<Entity>,
    pub pid: PID2D,
    pub with_velocity_lookahead: bool,
    pub with_rotation_lookahead: bool,
}

impl Default for FollowEntityMovement {
    fn default() -> Self {
        Self {
            target: None,
            pid: PID2D::new(
                PID::basic(1.0, 0.0, 0.0, 0.0),
                PID::basic(1.0, 0.0, 0.0, 0.0),
            ),
            with_velocity_lookahead: false,
            with_rotation_lookahead: false,
        }
    }
}

impl FollowEntityMovement {
    pub fn new(target: Option<Entity>, pid: PID2D) -> Self {
        Self {
            target,
            pid,
            ..default()
        }
    }

    pub fn basic(target: Entity) -> Self {
        Self {
            target: Some(target),
            pid: PID2D::new(
                PID::basic(1.0, 0.0, 0.0, 0.0),
                PID::basic(1.0, 0.0, 0.0, 0.0),
            ),
            ..default()
        }
    }

    pub fn smooth(target: Entity) -> Self {
        Self {
            target: Some(target),
            pid: PID2D::new(
                PID::basic(0.05, 0.1, 0.0, 0.0),
                PID::basic(0.05, 0.1, 0.0, 0.0),
            ),
            with_velocity_lookahead: true,
            with_rotation_lookahead: true,
        }
    }

    pub fn set_target(&mut self, target: Entity) {
        self.target = Some(target);
    }

    pub fn no_target(&mut self) {
        self.target = None;
    }
}

////////////////////////////////////////////////////////////////////////////////
/// Systems
////////////////////////////////////////////////////////////////////////////////

pub fn update(
    time: Res<Time>,
    mut target_query: Query<
        (
            &Transform,
            Option<&Velocity>,
            Option<&ActionState<PlayerShipAction>>,
        ),
        Without<FollowEntityMovement>,
    >,
    mut query: Query<(&mut Transform, &mut FollowEntityMovement)>,
) {
    if time.delta_seconds() == 0.0 {
        return;
    }

    for (mut transform, mut follow_entity_movement) in query.iter_mut() {
        if let Some((target_transform, target_velocity_opt, input_action_opt)) =
            follow_entity_movement
                .target
                .and_then(|target| target_query.get_mut(target).ok())
        {
            // Determine desired camera placement

            let mut desired_camera_placement = target_transform.translation.xy();

            if follow_entity_movement.with_velocity_lookahead {
                let linvel = target_velocity_opt
                    .map(|vel| vel.linvel)
                    .unwrap_or(Vec2::ZERO);
                desired_camera_placement += linvel.clamp_length_max(100.0);
            }

            if follow_entity_movement.with_rotation_lookahead {
                let player_rotation = if let Some(axis_data) = input_action_opt
                    .and_then(|action| action.clamped_axis_pair(PlayerShipAction::RotateShip))
                {
                    100.0 * axis_data.xy()
                } else {
                    target_transform.rotation.mul_vec3(Vec3::Y).xy().normalize() * 100.0
                };

                desired_camera_placement += player_rotation.clamp_length_max(100.0);
            }

            follow_entity_movement
                .pid
                .set_setpoint(desired_camera_placement);

            // Determine desired camera zoom level

            // Move camera

            let control_signal = follow_entity_movement
                .pid
                .update(transform.translation.xy(), time.delta_seconds());

            transform.translation += Vec3::new(control_signal.x, control_signal.y, 0.0);
        }
    }
}

fn debug_position(mut gizmos: Gizmos, query: Query<&Transform, With<FollowEntityMovement>>) {
    for transform in query.iter() {
        // Crosshair for the camera's current position
        gizmos::crosshair(&mut gizmos, &transform.translation.xy(), Color::BLUE, 10.0);
    }
}

fn debug_setpoint(mut gizmos: Gizmos, query: Query<&FollowEntityMovement>) {
    for follow_entity in query.iter() {
        // Crosshair for the camera's setpoint
        gizmos::crosshair(
            &mut gizmos,
            &follow_entity.pid.get_setpoint(),
            Color::GREEN,
            10.0,
        );
    }
}
