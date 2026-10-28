use bevy::{
    feathers::controls::FeathersSlider,
    prelude::*,
    ui_widgets::{SliderPrecision, SliderStep, SliderValue, ValueChange, slider_self_update},
};

pub struct SimulationSettingsPlugin;

impl Plugin for SimulationSettingsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SimulationConfig>();
    }
}

pub fn config_ui() -> impl Scene {
    bsn! {
        #ConfigUi
        Node {
            width: percent(100),
            align_items: AlignItems::Start,
            display: Display::Flex,
            column_gap: px(8)
        }
        Children [
            Text::new("Simulation speed")
            --
            @FeathersSlider {
                @max: 10.0
            }
            SliderValue(1.0)
            SliderStep(0.1)
            SliderPrecision(2)
            on(slider_self_update)
            on(|event: On<ValueChange<f32>>, mut config: ResMut<SimulationConfig>| {
                config.speed_modifier = event.value;
            })
        ]
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
