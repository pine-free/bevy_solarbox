use bevy::prelude::*;

use crate::planets::{OrbitCenter, OrbitRadius, OrbitSpeed, PlanetPlugin};

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
        center: &'static OrbitCenter,
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
    ) -> Result {
        for (planet, orbit) in planets {
            let planet_tr = transforms.get(planet)?;
            let to_orbit_center = orbit.center.sub(planet_tr.translation.xy());
            let tangential_orbit = to_orbit_center.rotate_angle(f32::consts::FRAC_PI_2);
            let mut planet_tr = transforms.get_mut(planet)?;
            planet_tr.translation = -to_orbit_center.extend(0.).normalize() * **orbit.radius;
            planet_tr.translation += (tangential_orbit.normalize() * **orbit.speed).extend(0.);
        }
        Ok(())
    }
}

fn setup() -> impl SceneList {
    bsn_list! {
        Camera2d
        --
        #Planet1
        @planets::planet(20., Color::Lcha(Lcha::sequential_dispersed(1)))
        Transform::from_translation(Vec3::X)
        OrbitCenter(Vec2::ZERO)
        OrbitSpeed(2.)
        OrbitRadius(100.)
        --
        #Planet2
        @planets::planet(40., Color::Lcha(Lcha::sequential_dispersed(2)))
        Transform::from_translation(Vec3::X)
        OrbitCenter(Vec2::ZERO)
        OrbitSpeed(1.)
        OrbitRadius(215.)
        --
        #Planet3
        @planets::planet(15., Color::Lcha(Lcha::sequential_dispersed(3)))
        Transform::from_translation(Vec3::X)
        OrbitCenter(Vec2::ZERO)
        OrbitSpeed(0.5)
        OrbitRadius(432.)
    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PlanetPlugin))
        .add_systems(Startup, setup.spawn())
        .run();
}
