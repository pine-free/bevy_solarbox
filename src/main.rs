use bevy::prelude::*;

use crate::{
    planets::{OrbitAnchor, OrbitRadius, OrbitSpeed, PlanetPlugin},
    simulation_ui::SimulationUiPlugin,
};

mod planets;
mod simulation_settings;
mod simulation_ui;
mod bg_shader {
    use bevy::{
        prelude::*,
        render::render_resource::{AsBindGroup, ShaderType},
    };

    #[derive(Asset, TypePath, AsBindGroup, ShaderType, Debug, Clone)]
    #[uniform(0, BackgroundOrbitalsMaterial)]
    pub struct BackgroundOrbitalsMaterial {
        /// Radius, anchor position
        test: Vec2,
    }

    impl<'a> From<&'a BackgroundOrbitalsMaterial> for BackgroundOrbitalsMaterial {
        fn from(value: &'a BackgroundOrbitalsMaterial) -> Self {
            value.clone()
        }
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
        #Orbit3
        @planets::orbit(100., Color::WHITE.with_alpha(0.1), #Planet2)
        --
        #Orbit2
        @planets::orbit(215., Color::WHITE.with_alpha(0.1), #OrbitCenter)
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
