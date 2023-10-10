use bevy::prelude::*;
pub struct TraumaPlugin;

impl Plugin for TraumaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_trauma));
    }
}

// Amount of trauma the player has, which is used to shake the camera
// Trauma is added when the player takes damage, and decays linearly over time
#[derive(Component)]
pub struct Trauma {
    trauma: f32,
    trauma_decay: f32,
}

impl Trauma {
    pub fn new(trauma: f32, trauma_decay: f32) -> Self {
        Self {
            trauma,
            trauma_decay,
        }
    }

    pub fn default() -> Self {
        Self {
            trauma: 0.0,
            trauma_decay: 4.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.trauma -= self.trauma_decay * dt;
        self.trauma = self.trauma.max(0.0);
    }

    pub fn add_trauma(&mut self, trauma: f32) {
        self.trauma += trauma.min(1.0).max(0.0);
    }

    pub fn set_trauma(&mut self, trauma: f32) {
        self.trauma = trauma.min(1.0).max(0.0);
    }

    pub fn get_trauma(&self) -> f32 {
        self.trauma
    }
}

pub fn update_trauma(time: Res<Time>, mut query: Query<&mut Trauma>) {
    for mut trauma in query.iter_mut() {
        trauma.update(time.delta_seconds());
    }
}
