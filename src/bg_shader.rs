use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderType},
    sprite_render::{Material2d, Material2dPlugin},
};

use crate::{MainCamera, planets::OrbitQuery, simulation_settings::SimulationConfig};

#[derive(Debug, Clone, Copy, Component, Default)]
pub struct OrbitsDisplayPlane;

pub fn plugin(app: &mut App) {
    app.add_plugins(Material2dPlugin::<BackgroundOrbitalsMaterial>::default())
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            update_orbits_display.run_if(resource_changed::<SimulationConfig>),
        )
        .add_systems(PostUpdate, update_orbits);
}

fn setup(window: Single<&Window>, mut commands: Commands) {
    let size = window.width().max(window.height());
    commands.spawn_scene(
        bsn! {
            #OrbitsDisplayPlane
            OrbitsDisplayPlane
            Mesh2d(asset_value(Rectangle::default()))
            MeshMaterial2d<BackgroundOrbitalsMaterial>(asset_value(BackgroundOrbitalsMaterial::default()))
            Transform::from_scale(Vec3::splat(size).with_z(-2.0))
        }
    );
}

fn update_orbits_display(
    mut orbits_bg: Single<&mut Transform, With<OrbitsDisplayPlane>>,
    window: Single<&Window>,
    config: Res<SimulationConfig>,
) {
    let size = window.size().max_element();
    orbits_bg.translation = config.camera_position.extend(orbits_bg.translation.z);
    orbits_bg.scale = Vec3::splat(size * config.camera_zoom);
}

fn update_orbits(
    bg: Single<&MeshMaterial2d<BackgroundOrbitalsMaterial>>,
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
