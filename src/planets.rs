use std::f32;

use bevy::{
    ecs::{query::QueryData, template::EntityTemplate},
    prelude::*,
};

use crate::{
    MainCamera, bg_shader,
    planet_shader::{self, PlanetMaterial},
    simulation_settings::{SimulationConfig, SimulationSettingsPlugin},
};

#[derive(Default)]
pub struct PlanetPlugin {
    pub debug: bool,
}

impl Plugin for PlanetPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            SimulationSettingsPlugin,
            MeshPickingPlugin,
            bg_shader::plugin,
            planet_shader::plugin,
        ))
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
pub struct OrbitSpeed {
    pub value: f32,
    memorised_value: Option<f32>,
}

impl OrbitSpeed {
    pub fn new(value: f32) -> Self {
        Self { value, ..default() }
    }

    pub fn disable(&mut self) {
        if self.value == 0. {
            return;
        }
        self.memorised_value = Some(self.value);
        self.value = 0.;
    }

    pub fn restore(&mut self) {
        if let Some(v) = self.memorised_value {
            self.value = v;
        }
    }
}

impl From<f32> for OrbitSpeed {
    fn from(value: f32) -> Self {
        Self { value, ..default() }
    }
}

impl std::ops::Deref for OrbitSpeed {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.value
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
    pub speed: &'static OrbitSpeed,
    pub radius: &'static OrbitRadius,
    pub orbit_anchor: &'static OrbitAnchor,
}

pub fn planet(radius: f32, color: Color) -> impl Scene {
    bsn! {
        Planet
        Mesh2d(asset_value(Rectangle::from_length(radius * 2.)))
        MeshMaterial2d<PlanetMaterial>(asset_value(PlanetMaterial::new(color.to_linear().to_vec3(), radius, Vec2::ZERO)))
        on(|drag: On<PointerDragStart>, mut speeds: Query<&mut OrbitSpeed>| -> Result {
            info!("drag start");
            speeds.get_mut(drag.entity)?.disable();
            Ok(())
        })
        on(|drag: On<PointerDrag>,
            mut planets: Query<(&mut OrbitRadius, &OrbitAnchor), With<Planet>>,
            transforms: Query<&mut Transform>,
            camera: Single<(&Camera, &GlobalTransform), With<MainCamera>>| -> Result {
            let (camera, camera_tr) = *camera;
            let (mut radius, anchor) = planets.get_mut(drag.entity)?;
            let anchor_pos = transforms.get(**anchor)?.translation;
            let new_translation = camera.viewport_to_world_2d(camera_tr, drag.pointer.position)?.extend(0.);
            radius.0 = (new_translation - anchor_pos).length();
            Ok(())
        })
        on(|drag: On<PointerDragEnd>, mut speeds: Query<&mut OrbitSpeed>| -> Result {
            info!("drag end");
            speeds.get_mut(drag.entity)?.restore();
            Ok(())
        })
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
