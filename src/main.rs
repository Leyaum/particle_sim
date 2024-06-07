mod particle_world;
mod sim_state;
mod particle;
mod entity_map;

use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use crate::particle_world::ParticleWorld;
use crate::sim_state::WindowState;
use crate::entity_map::EntityMap;

fn main() {
    let mut world = ParticleWorld::default();
    let map_size = Vec2::new(150.0,100.0);

    let mut map = EntityMap::new(map_size, 10.0);
    let pos1 = Vec2::new(74.0,0.0);
    map.add_entity(1, pos1);
    //let pos2 = Vec2::new(-5., -5.);
    //map.add_entity(2, pos2);

    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    let color = Color::rgb(1.0,1.0,1.0);
    let shape = Mesh2dHandle(meshes.add(Circle{radius: 5.0}));
    let mesh = MaterialMesh2dBundle {
        mesh: shape,
        material: materials.add(color),
        transform: Transform::from_xyz(-49.0,-49.0,0.0),
        ..default()
    };
    commands.spawn(mesh);
}

fn update(

) {

}
