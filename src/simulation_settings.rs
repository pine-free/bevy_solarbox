use bevy::{
    feathers::{
        controls::{FeathersCheckbox, FeathersSlider},
        display::caption,
    },
    prelude::*,
    ui::Checked,
    ui_widgets::{SliderPrecision, SliderStep, SliderValue, ValueChange, slider_self_update},
};

use crate::{bg_shader::OrbitsDisplayPlane, planets::Orbit};

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
            flex_direction: FlexDirection::Column,
            column_gap: px(8),
            row_gap: px(8)
        }
        Children [
            Node {
                width: percent(100),
                flex_direction: FlexDirection::Column,
                row_gap: px(8),
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
            --
            Node {
                width: percent(100),
                flex_direction: FlexDirection::Column,
                row_gap: px(8),
            }
            Children [
                Text::new("Zoom")
                --
                @FeathersSlider {
                    @max: 5.0,
                    @min: 0.2
                }
                SliderValue(1.0)
                SliderStep(0.1)
                SliderPrecision(2)
                on(slider_self_update)
                on(|event: On<ValueChange<f32>>, camera: Single<&mut Projection, With<Camera>>| {
                    let mut projection = camera;
                    let Projection::Orthographic(ref mut orthographic) = **projection else {panic!("unsupported projection type")};
                    orthographic.scale = event.value;
                })
            ]
            --
            @FeathersCheckbox {
                @caption: bsn! { @caption("Show orbits") }
            }
            // FIXME if orbits start out invisible then they show up in front of
            // the planets for some reason...
            Checked
            on(|change: On<ValueChange<bool>>, mut commands: Commands, orbits: Single<Entity, With<OrbitsDisplayPlane>>| {
                if change.value {
                    commands.entity(*orbits).insert(Visibility::Inherited);
                    commands.entity(change.source).insert(Checked);
                } else {
                    commands.entity(*orbits).insert(Visibility::Hidden);
                    commands.entity(change.source).remove::<Checked>();
                }
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
