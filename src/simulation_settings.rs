use bevy::{
    feathers::{
        controls::{FeathersCheckbox, FeathersSlider},
        display::caption,
    },
    input::gestures::PinchGesture,
    prelude::*,
    ui::Checked,
    ui_widgets::{
        SetSliderValue, SliderPrecision, SliderStep, SliderValue, ValueChange, slider_self_update,
    },
};

use crate::{MainCamera, bg_shader::OrbitsDisplayPlane, globals, planets::Orbit};

pub struct SimulationSettingsPlugin;

impl Plugin for SimulationSettingsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SimulationConfig>()
            .add_systems(
                Update,
                set_camera_zoom
                    .chain()
                    .run_if(resource_changed::<SimulationConfig>),
            )
            .add_systems(Update, pinch_to_zoom.before(set_camera_zoom));
    }
}

fn pinch_to_zoom(mut pinch: MessageReader<PinchGesture>, mut config: ResMut<SimulationConfig>) {
    for PinchGesture(delta) in pinch.read() {
        config.camera_zoom =
            (config.camera_zoom * (1. + delta)).clamp(globals::ZOOM_MIN, globals::ZOOM_MAX);
    }
}

fn set_camera_zoom(
    projection: Single<&mut Projection, With<MainCamera>>,
    slider: Option<Single<Entity, With<ZoomSlider>>>,
    config: Res<SimulationConfig>,
    mut commands: Commands,
) {
    let mut projection = projection;
    let Projection::Orthographic(ref mut orthographic) = **projection else {
        panic!("unsupported projection type")
    };
    orthographic.scale = config.camera_zoom;

    if let Some(entity) = slider {
        commands.trigger(SetSliderValue {
            entity: *entity,
            change: bevy::ui_widgets::SliderValueChange::Absolute(config.camera_zoom),
        });
    }
}

#[derive(Debug, Clone, Copy, Component, Default)]
pub struct ZoomSlider;

pub fn slider(text: String, slider_elem: impl Scene) -> impl Scene {
    bsn! {
        Node {
            width: percent(100),
            flex_direction: FlexDirection::Column,
            row_gap: px(8),
        }
        Children [
            Text::new(text)
            --
            @slider_elem
            SliderValue(1.0)
            SliderStep(0.1)
            SliderPrecision(2)
            on(slider_self_update)
        ]
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
            @slider(
                "Simulation speed".to_string(),
                bsn! {
                    @FeathersSlider {
                        @max: 10.0
                    }
                    on(|event: On<ValueChange<f32>>, mut config: ResMut<SimulationConfig>| {
                        config.speed_modifier = event.value;
                    })
                }
            )
            --
            @slider(
                "Zoom".to_string(),
                bsn! {
                    ZoomSlider
                    @FeathersSlider {
                        @max: globals::ZOOM_MAX,
                        @min: globals::ZOOM_MIN
                    }
                    on(|event: On<ValueChange<f32>>, mut config: ResMut<SimulationConfig>| {
                        config.camera_zoom = event.value;
                    })
                }
            )
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
    pub camera_zoom: f32,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            speed_modifier: 1.,
            camera_zoom: globals::STARTING_ZOOM,
        }
    }
}
