#![warn(clippy::pedantic)]
use bevy::prelude::*;

use crate::{
    planets::{OrbitAnchor, OrbitRadius, OrbitSpeed, PlanetPlugin, PlanetRenderRadius},
    simulation_ui::SimulationUiPlugin,
};

mod bg_shader;
mod globals;
mod planet_shader;
mod planets;
mod simulation_settings;
mod simulation_ui;

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
        @planets::planet(Color::Lcha(Lcha::sequential_dispersed(1)))
        Transform::from_translation(Vec3::X * 115.)
        OrbitSpeed::new(0.1)
        OrbitRadius(115.)
        PlanetRenderRadius(30.)
        OrbitAnchor(#OrbitCenter)
        --
        #Planet2
        @planets::planet(Color::Lcha(Lcha::sequential_dispersed(2)))
        Transform::from_translation(Vec3::X * 215.)
        OrbitSpeed::new(0.1)
        OrbitRadius(215.)
        PlanetRenderRadius(40.)
        OrbitAnchor(#OrbitCenter)
        --
        #Planet3
        @planets::planet(Color::Lcha(Lcha::sequential_dispersed(3)))
        Transform::from_translation(Vec3::X * 100.)
        OrbitSpeed::new(1.)
        OrbitRadius(100.)
        PlanetRenderRadius(10.)
        OrbitAnchor(#Planet2)
        --
        #Planet4
        @planets::planet(Color::Lcha(Lcha::sequential_dispersed(4)))
        Transform::from_translation(Vec3::X * 70.)
        OrbitSpeed::new(-0.3)
        OrbitRadius(70.)
        PlanetRenderRadius(10.)
        OrbitAnchor(#Planet3)

    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PlanetPlugin, SimulationUiPlugin))
        .add_systems(Startup, setup.spawn())
        .run();
}
