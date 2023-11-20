use super::Scene;
use crate::game::{assets, background, camera};
use crate::utility_systems;
use bevy::prelude::*;
pub struct AssetsScenePlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Component)]
struct AssetLabel;

impl Plugin for AssetsScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(Scene::Assets),
            (background::spawn, camera::spawn, spawn),
        )
        .add_systems(
            OnExit(Scene::Assets),
            (
                background::despawn,
                camera::despawn,
                utility_systems::cleanup::<AssetLabel>,
            ),
        );
    }
}

fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
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
