use bevy::prelude::*;

use crate::planets::{Anchor, OrbitAnchor, OrbitCenter, OrbitRadius, OrbitSpeed, PlanetPlugin};

mod planets {
    use std::{f32, ops::Sub};

    use bevy::{ecs::query::QueryData, prelude::*};

    pub struct PlanetPlugin;

    impl Plugin for PlanetPlugin {
        fn build(&self, app: &mut App) {
            app.add_systems(FixedUpdate, move_orbits);
        }
    }

    #[derive(Clone, Copy, Debug, Component, Default)]
    pub struct Planet;

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

    fn move_orbits(
        planets: Populated<(Entity, OrbitQuery)>,
        mut transforms: Query<&mut Transform>,
        anchors: Populated<(Entity, &Anchor)>,
    ) -> Result {
        for (a, orb) in anchors {
            info!("anchor {}: {:?}", a, orb.0);
        }
        for (planet, orbit) in planets {
            info!("moving planet {planet}");
            let planet_tr = transforms.get(planet)?;
            let anchor_pos = *transforms.get(**orbit.orbit_anchor)?;
            let to_orbit_center = anchor_pos.translation.xy() - planet_tr.translation.xy();
            let tangential_orbit = to_orbit_center.rotate_angle(f32::consts::FRAC_PI_2);
            let mut planet_tr = transforms.get_mut(planet)?;
            info!("position {planet}: {:?}", planet_tr.translation);
            planet_tr.translation =
                -to_orbit_center.extend(0.).normalize() * **orbit.radius + anchor_pos.translation;
            planet_tr.translation += (tangential_orbit.normalize() * **orbit.speed).extend(0.);
        }
        Ok(())
    }
}

fn setup() -> impl SceneList {
    bsn_list! {
        Camera2d
        --
        #OrbitCenter
        Transform::from_translation(Vec3::ZERO)
        --
        #Planet2
        @planets::planet(40., Color::Lcha(Lcha::sequential_dispersed(2)))
        Transform::from_translation(Vec3::X)
        OrbitSpeed(1.)
        OrbitRadius(215.)
        OrbitAnchor(#OrbitCenter)
        --
        #Planet1
        @planets::planet(20., Color::Lcha(Lcha::sequential_dispersed(1)))
        Transform::from_translation(Vec3::X)
        OrbitSpeed(-2.)
        OrbitRadius(100.)
        OrbitAnchor(#OrbitCenter)
        --
        #Planet3
        @planets::planet(15., Color::Lcha(Lcha::sequential_dispersed(3)))
        Transform::from_translation(Vec3::X)
        OrbitSpeed(0.5)
        OrbitRadius(432.)
        OrbitAnchor(#OrbitCenter)
        --
        #Planet4
        @planets::planet(10., Color::Lcha(Lcha::sequential_dispersed(4)))
        Transform::from_translation(Vec3::X)
        OrbitSpeed(-3.)
        OrbitRadius(100.)
        OrbitAnchor(#Planet2)

    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PlanetPlugin))
        .add_systems(Startup, setup.spawn())
        .run();
}
