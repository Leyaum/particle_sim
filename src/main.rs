mod particle_world;
mod sim_state;
mod particle;

use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use crate::particle_world::ParticleWorld;
use crate:: sim_state::WindowState;

fn main() {
    let mut world = ParticleWorld::default();

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
    let shape = Mesh2dHandle(meshes.add(Circle{radius: 50.0}));
    let mesh = MaterialMesh2dBundle {
        mesh: shape,
        material: materials.add(color),
        transform: Transform::from_xyz(0.0,0.0,0.0),
        ..default()
    };
    commands.spawn(mesh);
}

fn update(

) {

}
