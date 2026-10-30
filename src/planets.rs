use std::f32;

use bevy::{
    ecs::{component::Mutable, query::QueryData},
    feathers::{
        containers::{group, group_body, group_header},
        controls::{FeathersNumberInput, NumberInputValue},
        display::{caption, label, label_small},
    },
    prelude::*,
    ui_widgets::ValueChange,
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
        .add_systems(FixedUpdate, (move_orbits))
        .add_systems(
            Update,
            (
                update_planet_radius,
                display_planet_stats,
                update_planet_stats_display,
            )
                .chain(),
        )
        .add_observer(observe_planet_select);
    }
}

#[derive(Clone, Copy, Debug, Component, Default)]
pub struct Planet;

#[derive(Debug, Clone, Component, FromTemplate)]
pub struct SelectedPlanet;

#[derive(Debug, Clone, Component, FromTemplate)]
#[relationship(relationship_target = StatsDisplayTargets)]
pub struct DisplayStatsIn(pub Entity);

#[derive(Debug, Clone, Component, FromTemplate)]
#[relationship_target(relationship = DisplayStatsIn)]
pub struct StatsDisplayTargets(Entity);

#[derive(Clone, Copy, Debug, Component, Default)]
pub struct PlanetRenderRadius(pub f32);

impl std::ops::Deref for PlanetRenderRadius {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

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
    pub angular_speed: &'static OrbitSpeed,
    pub planet_radius: &'static PlanetRenderRadius,
    pub orbit_radius: &'static OrbitRadius,
    pub orbit_anchor: &'static OrbitAnchor,
}

#[derive(Clone, Copy, Debug, Default, Component)]
pub struct SelectedPlanetInfo;

#[derive(Clone, Copy, Debug, Component, FromTemplate)]
pub struct PlanetStatsWidgets {
    pub size_radius: Entity,
    pub orbit_radius: Entity,
    pub angular_speed: Entity,
}

fn change_planet_stats<C, T>(
    change: On<ValueChange<T>>,
    info: Single<&StatsDisplayTargets, With<SelectedPlanetInfo>>,
    mut query: Query<&mut C>,
) -> Result
where
    T: Copy,
    ValueChange<T>: Event,
    C: Component<Mutability = Mutable> + From<T>,
{
    let planet = info.0;
    let mut param = query.get_mut(planet)?;
    *param = C::from(change.value);
    Ok(())
}

pub fn planet_info() -> impl Scene {
    bsn! {
        SelectedPlanetInfo
        PlanetStatsWidgets {
            size_radius: #SizeRadius,
            orbit_radius: #OrbitRadius,
            angular_speed: #AngularSpeed
        }
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
                #SizeRadius
                @FeathersNumberInput
                on(change_planet_stats::<PlanetRenderRadius, f32>)
                --
                @label_small("Orbit Radius")
                --
                #OrbitRadius
                @FeathersNumberInput
                on(change_planet_stats::<OrbitRadius, f32>)
                --
                @label_small("Angular speed")
                --
                #AngularSpeed
                @FeathersNumberInput
                on(change_planet_stats::<OrbitSpeed, f32>)
            ]
        ]
    }
}

pub fn planet(color: Color) -> impl Scene {
    bsn! {
        Planet
        Mesh2d(asset_value(Rectangle::default()))
        MeshMaterial2d<PlanetMaterial>(asset_value(PlanetMaterial::new(color.to_linear().to_vec3(), 1.0, Vec2::ZERO)))
        on(|drag: On<PointerDragStart>, mut speeds: Query<&mut OrbitSpeed>| -> Result {
            speeds.get_mut(drag.entity)?.disable();
            Ok(())
        })
        on(|drag: On<PointerDrag>,
            anchors: Query<&OrbitAnchor>,
            radii: Query<&PlanetRenderRadius>,
            mut planets: Query<&mut OrbitRadius, With<Planet>>,
            transforms: Query<&mut Transform>,
            camera: Single<(&Camera, &GlobalTransform), With<MainCamera>>| -> Result {
            let (camera, camera_tr) = *camera;
            let anchor_entity = **anchors.get(drag.entity)?;
            let anchor_radius = radii.get(anchor_entity).map_or_default(|r| r.0);
            let mut radius = planets.get_mut(drag.entity)?;
            let anchor_pos = transforms.get(anchor_entity)?.translation;
            let new_translation = camera.viewport_to_world_2d(camera_tr, drag.pointer.position)?.extend(0.);
            radius.0 = (new_translation - anchor_pos).length() - anchor_radius;
            Ok(())
        })
        on(|click: On<PointerClick>| {
            info!("planet {} clicked", click.entity);
        })
        on(|drag: On<PointerDragEnd>, mut speeds: Query<&mut OrbitSpeed>| -> Result {
            speeds.get_mut(drag.entity)?.restore();
            Ok(())
        })
    }
}

fn update_planet_radius(
    planets: Query<PlanetInfoQuery, Changed<PlanetRenderRadius>>,
    mut transforms: Query<&mut Transform>,
) -> Result {
    for planet in planets.iter() {
        transforms.get_mut(planet.entity)?.scale = Vec3::splat(**planet.planet_radius * 2.);
    }
    Ok(())
}

fn move_orbits(
    planets: Populated<(Entity, PlanetInfoQuery), With<Planet>>,
    radii: Populated<&PlanetRenderRadius>,
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
        planet_tr.translation = -to_orbit_center.extend(0.).normalize()
            * (**orbit.orbit_radius
                + radii
                    .get(**orbit.orbit_anchor)
                    .map(|r| r.0)
                    .unwrap_or_default())
            + anchor_pos.translation;
        // gizmos.arrow_2d(
        //     planet_tr.translation.xy(),
        //     planet_tr.translation.xy() + tangential_orbit.normalize() * **orbit.speed * 20.,
        //     LinearRgba::RED,
        // );
        planet_tr.translation +=
            (tangential_orbit.normalize() * **orbit.angular_speed * settings.speed_modifier)
                .extend(0.);
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
    info!("clicked entity {:#?}", ev);
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

fn display_planet_stats(
    planets: Query<PlanetInfoQuery, Changed<SelectedPlanet>>,
    info_display: Single<Entity, With<SelectedPlanetInfo>>,
    mut commands: Commands,
) {
    planets.iter().for_each(|pl| {
        commands
            .entity(pl.entity)
            .insert(DisplayStatsIn(*info_display));
    });
}

fn update_planet_stats_display(
    info_display: Single<(&PlanetStatsWidgets, &StatsDisplayTargets)>,
    planets: Query<PlanetInfoQuery>,
    mut commands: Commands,
) -> Result {
    let (
        PlanetStatsWidgets {
            size_radius,
            orbit_radius,
            angular_speed,
        },
        StatsDisplayTargets(t_planet),
    ) = *info_display;

    let target_planet = planets.get(*t_planet)?;
    commands
        .entity(*size_radius)
        .insert(NumberInputValue::F32(**target_planet.planet_radius));
    commands
        .entity(*orbit_radius)
        .insert(NumberInputValue::F32(**target_planet.orbit_radius));
    commands
        .entity(*angular_speed)
        .insert(NumberInputValue::F32(**target_planet.angular_speed));

    Ok(())
}
