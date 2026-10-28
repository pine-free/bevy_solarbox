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
        sprite_render::{Material2d, Material2dPlugin},
    };

    use crate::planets::OrbitQuery;

    pub fn plugin(app: &mut App) {
        app.add_plugins(Material2dPlugin::<BackgroundOrbitalsMaterial>::default())
            .add_systems(Startup, setup)
            .add_systems(PostUpdate, update_orbits);
    }

    fn setup(window: Single<&Window>, mut commands: Commands) {
        let size = window.width().max(window.height());
        commands.spawn_scene(
            bsn! {
                Mesh2d(asset_value(Rectangle::default()))
                MeshMaterial2d<BackgroundOrbitalsMaterial>(asset_value(BackgroundOrbitalsMaterial::default()))
                Transform::from_scale(Vec3::splat(size).with_z(-2.0))
            }
        );
    }

    fn update_orbits(
        mut bg: Single<&MeshMaterial2d<BackgroundOrbitalsMaterial>>,
        mut material: ResMut<Assets<BackgroundOrbitalsMaterial>>,
        orbits: Query<OrbitQuery>,
        transforms: Query<&Transform>,
    ) -> Result {
        let mut material = material
            .get_mut(&bg.0)
            .context("failed to get background material")?;

        let mut orbit_data = vec![];
        for orbit in orbits {
            let r = *orbit.radius;
            let anchor_pos = transforms.get(**orbit.orbit_anchor)?.translation.xy();
            orbit_data.push(anchor_pos.extend(*r));
        }

        let v = orbit_data
            .into_iter()
            .chain([Vec3::NEG_ONE].into_iter().cycle())
            .take(128)
            .collect::<Vec<_>>();

        material.orbits_data = v
            .as_array::<128>()
            .cloned()
            .context("got invalid number of orbit data")?;

        Ok(())
    }

    #[derive(Asset, TypePath, AsBindGroup, ShaderType, Debug, Clone)]
    #[uniform(0, BackgroundOrbitalsMaterial)]
    pub struct BackgroundOrbitalsMaterial {
        /// anchor position (x, y), radius (z)
        orbits_data: [Vec3; 128],
    }

    impl Default for BackgroundOrbitalsMaterial {
        fn default() -> Self {
            Self {
                orbits_data: [Vec3::NEG_X; 128],
            }
        }
    }

    impl Material2d for BackgroundOrbitalsMaterial {
        fn fragment_shader() -> bevy::shader::ShaderRef {
            "orbits.wesl".into()
        }
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
