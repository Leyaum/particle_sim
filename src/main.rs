mod particle_world;
mod sim_state;
mod particle;
mod entity_map;
mod rigid_body;

use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use crate::particle_world::ParticleWorld;
use crate::entity_map::EntityMap;
use crate::particle::Particle;
use crate::rigid_body::RigidBody;

fn main() {
    let mut world = ParticleWorld::default();
    let map_size = Vec2::new(100.0,100.0);

    let mut map = EntityMap::new(map_size, 10.0);

    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Startup, test)
        .insert_resource::<EntityMap>(map)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    let p1 = commands
        .spawn(Particle::default())
        .insert(MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle{radius:5.0})),
            material: materials.add(Color::rgb(1.0,1.0,1.0)),
            ..default()
        });
}

fn test(mut entity_map: ResMut<EntityMap>) {
    let pos1 = Vec2::new(0.0,0.0);
    entity_map.add_entity(1, pos1);
    let pos2 = Vec2::new(-5., -5.);
    entity_map.add_entity(2, pos2);
    entity_map.print_filled_containers();
}
