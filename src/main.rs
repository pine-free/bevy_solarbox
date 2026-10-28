use bevy::prelude::*;

use crate::planets::{OrbitAnchor, OrbitRadius, OrbitSpeed, PlanetPlugin};

mod planets;
mod simulation_settings {
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
}

fn setup() -> impl SceneList {
    bsn_list! {
        Camera2d
        --
        #OrbitCenter
        Transform::from_translation(Vec3::ZERO)
        --
        // #Orbit3
        // @planets::orbit(100., Color::WHITE.with_alpha(0.1), #Planet2)
        // --
        // #Orbit2
        // @planets::orbit(215., Color::WHITE.with_alpha(0.1), #OrbitCenter)
        // --
        #Planet2
        @planets::planet(40., Color::Lcha(Lcha::sequential_dispersed(2)))
        Transform::from_translation(Vec3::X * 215.)
        OrbitSpeed(0.1)
        OrbitRadius(215.)
        OrbitAnchor(#OrbitCenter)
        --
        #Planet3
        @planets::planet(10., Color::Lcha(Lcha::sequential_dispersed(3)))
        Transform::from_translation(Vec3::X * 100.)
        OrbitSpeed(1.)
        OrbitRadius(100.)
        OrbitAnchor(#Planet2)

    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PlanetPlugin::default()))
        .add_systems(Startup, setup.spawn())
        .run();
}
