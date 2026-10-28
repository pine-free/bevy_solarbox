use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderType};
use bevy::sprite_render::{Material2d, Material2dPlugin};

pub fn plugin(app: &mut App) {
    app.add_plugins(Material2dPlugin::<PlanetMaterial>::default())
        .add_systems(PostUpdate, update_planet_materials);
}

fn update_planet_materials(
    planets: Populated<(&MeshMaterial2d<PlanetMaterial>, &Transform)>,
    mut material: ResMut<Assets<PlanetMaterial>>,
) -> Result {
    for (mat, transform) in planets {
        let mut material = material
            .get_mut(mat)
            .context("failed to get planet material")?;

        material.position = transform.translation.xy();
    }

    Ok(())
}

#[derive(Asset, TypePath, AsBindGroup, ShaderType, Debug, Clone, Default)]
#[uniform(1, PlanetMaterial)]
pub struct PlanetMaterial {
    color: Vec3,
    radius: f32,
    position: Vec2,
}

impl PlanetMaterial {
    pub fn new(color: Vec3, radius: f32, position: Vec2) -> Self {
        Self {
            color,
            radius,
            position,
        }
    }
}

impl Material2d for PlanetMaterial {
    fn fragment_shader() -> bevy::shader::ShaderRef {
        "planet_material.wesl".into()
    }
}

impl<'a> From<&'a PlanetMaterial> for PlanetMaterial {
    fn from(value: &'a PlanetMaterial) -> Self {
        value.clone()
    }
}
