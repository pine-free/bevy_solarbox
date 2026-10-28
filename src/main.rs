use bevy::prelude::*;

use crate::{
    planets::{OrbitAnchor, OrbitRadius, OrbitSpeed, PlanetPlugin},
    simulation_ui::SimulationUiPlugin,
};

mod planets;
mod simulation_ui {
    use bevy::{
        feathers::{
            FeathersPlugins,
            dark_theme::create_dark_theme,
            theme::{ThemeBackgroundColor, UiTheme},
            tokens,
        },
        input_focus::tab_navigation::TabGroup,
        prelude::*,
    };

    pub struct SimulationUiPlugin;

    impl Plugin for SimulationUiPlugin {
        fn build(&self, app: &mut App) {
            app.add_plugins(FeathersPlugins)
                .insert_resource(UiTheme(create_dark_theme()))
                .add_systems(Startup, ui_root.spawn());
        }
    }

    #[derive(Default, Clone, Copy, Debug, Component)]
    pub struct UiRoot;

    fn ui_root() -> impl Scene {
        bsn! {
            UiRoot
            Node {
                width: percent(40),
                height: percent(40),
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Start,
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                column_gap: px(8),
            }
            TabGroup
            ThemeBackgroundColor(tokens::WINDOW_BG)
        }
    }
}
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
        .add_plugins((DefaultPlugins, PlanetPlugin::default(), SimulationUiPlugin))
        .add_systems(Startup, setup.spawn())
        .run();
}
