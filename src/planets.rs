use std::f32;

use bevy::{
    ecs::query::QueryData,
    feathers::{
        containers::{group, group_body, group_header},
        controls::FeathersNumberInput,
        display::{caption, label, label_small},
    },
    prelude::*,
};

use crate::{
    MainCamera, bg_shader,
    planet_shader::{self, PlanetMaterial},
    simulation_settings::{ConfigSidebar, SimulationConfig, SimulationSettingsPlugin},
};

#[derive(Default)]
pub struct PlanetPlugin;

impl Plugin for PlanetPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            SimulationSettingsPlugin,
            MeshPickingPlugin,
            bg_shader::plugin,
            planet_shader::plugin,
        ))
        .add_systems(FixedUpdate, move_orbits)
        .add_observer(observe_planet_select);
    }
}

#[derive(Clone, Copy, Debug, Component, Default)]
pub struct Planet;

#[derive(Debug, Clone, Component, Default)]
pub struct SelectedPlanet;

#[derive(Clone, Copy, Debug, Component, Default)]
pub struct PlanetRenderRadius(pub f32);

impl From<f32> for PlanetRenderRadius {
    fn from(value: f32) -> Self {
        Self(value)
    }
}

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

#[derive(Clone, Copy, Debug, Component, FromTemplate)]
#[relationship(relationship_target = Anchor)]
pub struct OrbitAnchor(pub Entity);

impl std::ops::Deref for OrbitAnchor {
    type Target = Entity;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(QueryData)]
#[query_data(derive(Debug))]
pub struct PlanetInfoQuery {
    pub entity: Entity,
    pub speed: &'static OrbitSpeed,
    pub orbit_radius: &'static OrbitRadius,
    pub orbit_anchor: &'static OrbitAnchor,
}

#[derive(Clone, Copy, Debug, Default, Component)]
pub struct SelectedPlanetInfo;

#[derive(Clone, Copy, Debug, Default, Component)]
pub struct SelectedPlanetInfoSizeRadius;

#[derive(Clone, Copy, Debug, Default, Component)]
pub struct SelectedPlanetInfoOrbitRadius;

#[derive(Clone, Copy, Debug, Default, Component)]
pub struct SelectedPlanetInfoAngularSpeed;

pub fn planet_info(orbit_radius: f32, size: f32, angular_speed: f32) -> impl Scene {
    bsn! {
        SelectedPlanetInfo
        @group()
        Children [
            @group_header()
            Children [
                @caption("Planet Info")
            ]
            --
            @group_body()
            Children [
                @label("Planet Parameters")
                --
                @label_small("Size Radius")
                --
                SelectedPlanetInfoSizeRadius
                @FeathersNumberInput
                --
                @label_small("Orbit Radius")
                --
                SelectedPlanetInfoOrbitRadius
                @FeathersNumberInput
                --
                @label_small("Angular speed")
                --
                SelectedPlanetInfoAngularSpeed
                @FeathersNumberInput
            ]
        ]
    }
}

pub fn planet(radius: f32, color: Color) -> impl Scene {
    bsn! {
        Planet
        Mesh2d(asset_value(Rectangle::from_length(radius * 2.)))
        MeshMaterial2d<PlanetMaterial>(asset_value(PlanetMaterial::new(color.to_linear().to_vec3(), radius, Vec2::ZERO)))
        on(|drag: On<PointerDragStart>, mut speeds: Query<&mut OrbitSpeed>| -> Result {
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
            speeds.get_mut(drag.entity)?.restore();
            Ok(())
        })
    }
}

fn move_orbits(
    planets: Populated<(Entity, PlanetInfoQuery), With<Planet>>,
    settings: Res<SimulationConfig>,
    mut transforms: Query<&mut Transform>,
    _gizmos: Gizmos,
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
            -to_orbit_center.extend(0.).normalize() * **orbit.orbit_radius + anchor_pos.translation;
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

fn observe_planet_select(
    click: On<PointerClick>,
    planets: Query<Entity, With<Planet>>,
    selected_planets: Query<Entity, With<SelectedPlanet>>,
    mut commands: Commands,
) {
    let ev = click.original_event_target();
    let is_planet = planets.get(ev).is_ok();
    let is_selected = selected_planets.get(ev).is_ok();
    info!("click detected");

    // If we chose the planet that has already been selected
    // then do nothing
    if is_planet && is_selected {
        info!("the clicked object is a selected planet, finishing");
        return;
    }

    info!("cleaning out all selected planets");
    // Clean all selected planets if we click off one
    selected_planets.iter().for_each(|p| {
        commands.entity(p).remove::<SelectedPlanet>();
    });

    if !is_planet {
        info!("selected object is not a planet, finishing");
        return;
    }

    info!("select planet {ev}, inserting selected component");
    commands.entity(ev).insert(SelectedPlanet);
}
