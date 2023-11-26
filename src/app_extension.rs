// use std::marker::Tuple;

use bevy::ecs::schedule::common_conditions::run_once as run_once_condition;
use bevy::ecs::schedule::run_enter_schedule;
use bevy::prelude::*;
use bevy::utils::HashSet;

use crate::scene::GameScene;

pub trait AppExtension {
    fn init_state<S: States>(&mut self, initial_state: S) -> &mut Self;

    fn add_game_scene<S, Marker>(&mut self, scene: GameScene, spawn: S)
    where
        S: IntoSystem<(), HashSet<Entity>, Marker>;
}

impl AppExtension for App {
    // Remove when [add_state is added](https://github.com/bevyengine/bevy/issues/10731)
    fn init_state<S: States>(&mut self, initial_state: S) -> &mut Self {
        self.insert_resource::<State<S>>(State::new(initial_state))
            .init_resource::<NextState<S>>()
            .add_systems(
                StateTransition,
                (
                    run_enter_schedule::<S>.run_if(run_once_condition()),
                    apply_state_transition::<S>,
                )
                    .chain(),
            );

        // The OnEnter, OnExit, and OnTransition schedules are lazily initialized
        // (i.e. when the first system is added to them), and World::try_run_schedule is used to fail
        // gracefully if they aren't present.
        self
    }

    /// An attempt at universalizing how we add game scenes. Unfortunately, it doesn't work.
    /// Since not some enitties are spawned dynamically, and some things that need to be cleaned up
    /// are not entities, they could be resources.
    fn add_game_scene<S, Marker>(&mut self, scene: GameScene, spawn: S)
    where
        S: IntoSystem<(), HashSet<Entity>, Marker>,
    {
        self.add_systems(OnEnter(scene), spawn.pipe(tag(scene)))
            .add_systems(OnExit(scene), cleanup(scene));
    }
}

fn tag<T: Component + PartialEq + Clone>(label: T) -> impl Fn(In<HashSet<Entity>>, Commands) {
    return move |In(entities), mut commands| {
        for entity in entities {
            commands.entity(entity).insert(label.clone());
        }
    };
}

fn cleanup<T: Component + PartialEq>(label: T) -> impl Fn(Commands, Query<(Entity, &T)>) {
    return move |mut commands, query| {
        for (entity, entity_label) in query.iter() {
            if *entity_label == label {
                commands.entity(entity).despawn_recursive();
            }
        }
    };
}
