use std::f32;

use bevy::{
    ecs::{query::QueryData, template::EntityTemplate},
    prelude::*,
};

use crate::simulation_settings::{SimulationConfig, SimulationSettingsPlugin};

#[derive(Default)]
pub struct PlanetPlugin {
    pub debug: bool,
}

impl Plugin for PlanetPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((SimulationSettingsPlugin, MeshPickingPlugin))
            .add_systems(FixedUpdate, move_orbits);
    }
}

#[derive(Clone, Copy, Debug, Component, Default)]
pub struct Planet;

#[derive(Clone, Copy, Debug, Component, Default)]
pub struct Orbit;

#[derive(Clone, Debug, Component)]
#[require(Transform)]
#[relationship_target(relationship = OrbitAnchor)]
pub struct Anchor(Vec<Entity>);

impl std::ops::Deref for Anchor {
    type Target = Vec<Entity>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, Component, Default)]
pub struct OrbitSpeed(pub f32);

impl From<f32> for OrbitSpeed {
    fn from(value: f32) -> Self {
        Self(value)
    }
}

impl std::ops::Deref for OrbitSpeed {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, Component, Default)]
pub struct OrbitRadius(pub f32);

impl From<f32> for OrbitRadius {
    fn from(value: f32) -> Self {
        Self(value)
    }
}

impl std::ops::Deref for OrbitRadius {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, Component, Default)]
pub struct OrbitCenter(pub Vec2);

#[derive(Clone, Copy, Debug, Component, FromTemplate)]
#[relationship(relationship_target = Anchor)]
pub struct OrbitAnchor(pub Entity);

impl std::ops::Deref for OrbitAnchor {
    type Target = Entity;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Vec2> for OrbitCenter {
    fn from(value: Vec2) -> Self {
        Self(value)
    }
}

impl std::ops::Deref for OrbitCenter {
    type Target = Vec2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(QueryData)]
#[query_data(derive(Debug))]
pub struct OrbitQuery {
    speed: &'static OrbitSpeed,
    radius: &'static OrbitRadius,
    orbit_anchor: &'static OrbitAnchor,
}

pub fn planet(radius: f32, color: Color) -> impl Scene {
    bsn! {
        Planet
        Mesh2d(asset_value(Circle::new(radius)))
        MeshMaterial2d<ColorMaterial>(asset_value(color))
    }
}

pub fn orbit(radius: f32, color: Color, anchor: EntityTemplate) -> impl Scene {
    bsn! {
        Orbit
        Mesh2d(asset_value(Annulus::new(radius - 1., radius + 1.)))
        MeshMaterial2d<ColorMaterial>(asset_value(color))
        Transform::default().with_translation(Vec3::ZERO.with_z(-1.))
        ChildOf(anchor)
    }
}

fn move_orbits(
    planets: Populated<(Entity, OrbitQuery), With<Planet>>,
    settings: Res<SimulationConfig>,
    mut transforms: Query<&mut Transform>,
    mut gizmos: Gizmos,
) -> Result {
    for (planet, orbit) in planets {
        // info!("moving planet {planet}");
        let planet_tr = transforms.get(planet)?;
        let anchor_pos = *transforms.get(**orbit.orbit_anchor)?;
        let to_orbit_center = anchor_pos.translation.xy() - planet_tr.translation.xy();
        let tangential_orbit = to_orbit_center.rotate_angle(f32::consts::FRAC_PI_2);
        // info!("after rotation: {:?}", tangential_orbit);
        let mut planet_tr = transforms.get_mut(planet)?;
        // info!("position {planet}: {:?}", planet_tr.translation);
        planet_tr.translation =
            -to_orbit_center.extend(0.).normalize() * **orbit.radius + anchor_pos.translation;
        // gizmos.arrow_2d(
        //     planet_tr.translation.xy(),
        //     planet_tr.translation.xy() + tangential_orbit.normalize() * **orbit.speed * 20.,
        //     LinearRgba::RED,
        // );
        planet_tr.translation +=
            (tangential_orbit.normalize() * **orbit.speed * settings.speed_modifier).extend(0.);
    }
    Ok(())
}
