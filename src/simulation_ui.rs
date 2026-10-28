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
            width: percent(25),
            height: percent(100),
            align_items: AlignItems::Start,
            justify_content: JustifyContent::Start,
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            column_gap: px(8),
        }
        TabGroup
        ThemeBackgroundColor(tokens::WINDOW_BG)
        Children [
            @crate::simulation_settings::config_ui()
        ]
    }
}
