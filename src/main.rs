use bevy::prelude::*;

use crate::{
    planets::{OrbitAnchor, OrbitRadius, OrbitSpeed, PlanetPlugin},
    simulation_ui::SimulationUiPlugin,
};

mod bg_shader;
mod globals;
mod planet_shader;
mod planets;
mod simulation_settings;
mod simulation_ui;
mod input {
    use bevy::ecs::event::Event;
    use bevy::input::gestures::PinchGesture;
    use bevy::platform::collections::HashMap;
    use bevy::prelude::*;

    #[derive(Clone, Copy, Debug, Event)]
    pub enum UserAction {}

    #[derive(Clone, Debug, Component)]
    pub struct Keymap<T: Event> {
        mappings: HashMap<T, KeyCode>,
    }
}

#[derive(Default, Clone, Copy, Debug, Component)]
pub struct MainCamera;

fn setup() -> impl SceneList {
    bsn_list! {
        #MainCamera
        Camera2d
        MainCamera
        --
        #OrbitCenter
        Transform::from_translation(Vec3::ZERO)
        --
        #Planet1
        @planets::planet(30., Color::Lcha(Lcha::sequential_dispersed(1)))
        Transform::from_translation(Vec3::X * 115.)
        OrbitSpeed::new(0.1)
        OrbitRadius(115.)
        OrbitAnchor(#OrbitCenter)
        --
        #Planet2
        @planets::planet(40., Color::Lcha(Lcha::sequential_dispersed(2)))
        Transform::from_translation(Vec3::X * 215.)
        OrbitSpeed::new(0.1)
        OrbitRadius(215.)
        OrbitAnchor(#OrbitCenter)
        --
        #Planet3
        @planets::planet(10., Color::Lcha(Lcha::sequential_dispersed(3)))
        Transform::from_translation(Vec3::X * 100.)
        OrbitSpeed::new(1.)
        OrbitRadius(100.)
        OrbitAnchor(#Planet2)

    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PlanetPlugin::default(), SimulationUiPlugin))
        .add_systems(Startup, setup.spawn())
        .run();
}
