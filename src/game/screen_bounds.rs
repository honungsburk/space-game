use bevy::{prelude::*, window::PrimaryWindow};

////////////////////////////////////////////////////////////////////////////////
// Plugin
////////////////////////////////////////////////////////////////////////////////

pub struct ScreenBoundsPlugin;

impl Plugin for ScreenBoundsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ScreenBounds>()
            .add_systems(Update, update_screen_bounds);
    }
}

////////////////////////////////////////////////////////////////////////////////
// Resource
////////////////////////////////////////////////////////////////////////////////

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
    window_query: Query<&Window, (With<PrimaryWindow>, Without<Camera>)>,
    camera_query: Query<&Transform, (Without<PrimaryWindow>, With<Camera>)>,
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
