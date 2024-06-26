use super::GameScene;
use crate::game::movement::KeyboardMovementBundle;
use crate::game::{assets, background};
use crate::utility_systems;
use bevy::prelude::*;

pub struct AssetsScenePlugin;

// pub fn add_game_scene<S, Marker>(app: &mut App, scene: GameScene, spawn: S)
// where
//     // S: System<In = In, Out = HashSet<Entity>>,
//     S: IntoSystem<(), HashSet<Entity>, Marker>,
// {

#[derive(Debug, Clone, Eq, PartialEq, Hash, Component)]
struct AssetLabel;

impl Plugin for AssetsScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameScene::Assets), (background::spawn, spawn))
            .add_systems(
                OnExit(GameScene::Assets),
                (
                    background::despawn,
                    utility_systems::cleanup::<AssetLabel>,
                    utility_systems::cleanup::<Camera>,
                ),
            );
    }
}
fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn((
        Camera2dBundle::default(),
        KeyboardMovementBundle::new(500.0),
    ));

    // Assets

    let mut y_offset = 0.0;

    for ass in assets::ALL {
        let mut x_offset = 0.0;
        for a in ass {
            // let name: &str = (**a).name;
            let sprite_path = (**a).sprite_path;
            let collider = (**a).collider();
            let spawn_transform = Transform::from_xyz(x_offset, y_offset, 0.0);

            commands.spawn((
                AssetLabel,
                SpriteBundle {
                    transform: spawn_transform,
                    texture: asset_server.load(sprite_path),
                    ..default()
                },
                collider,
            ));

            x_offset += 120.0;
        }
        y_offset += 120.0;
    }
}
