use crate::game::SimulationState;
use bevy::prelude::*;

use super::vitality::*;

pub fn pause_simulation(mut simulation_state_next_state: ResMut<NextState<SimulationState>>) {
    simulation_state_next_state.set(SimulationState::Paused);
}

pub fn resume_simulation(mut simulation_state_next_state: ResMut<NextState<SimulationState>>) {
    simulation_state_next_state.set(SimulationState::Running);
}

pub fn toggle_simulation(
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    mut simulation_state_next_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        match *simulation_state.get() {
            SimulationState::Running => {
                simulation_state_next_state.set(SimulationState::Paused);
                println!("Simulation Paused.");
            }
            SimulationState::Paused => {
                simulation_state_next_state.set(SimulationState::Running);
                println!("Simulation Running.");
            }
        }
    }
}
