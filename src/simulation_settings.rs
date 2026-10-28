use bevy::prelude::*;

pub struct SimulationSettingsPlugin;

impl Plugin for SimulationSettingsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SimulationConfig>();
    }
}

#[derive(Debug, Clone, Resource)]
pub struct SimulationConfig {
    pub speed_modifier: f32,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self { speed_modifier: 1. }
    }
}

impl SimulationConfig {
    pub fn new(speed_modifier: f32) -> Self {
        Self { speed_modifier }
    }
}
