mod particle_world;
mod sim_state;
mod particle;
mod entity_map;
mod rigid_body;

use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use crate::entity_map::{EntityMap, remap};
use crate::particle::Particle;
use crate::rigid_body::RigidBody;

fn main() {
    let map_size = Vec2::new(100.0,100.0);
    let map = EntityMap::new(map_size, 10.0);

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_systems(Startup, (
            setup,
        ))
        .add_systems(Update, (
            update,
            remap.after(update),
        ))
        .insert_resource::<EntityMap>(map)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut entity_map: ResMut<EntityMap>
) {
    commands.spawn(Camera2dBundle::default());

    let pos1 = Vec2::new(25.0, 25.0);
    add_particle(&mut commands, &mut meshes, &mut materials, &mut entity_map, pos1);

    let pos2 = Vec2::new(-25.0, -25.0);
    add_particle(&mut commands, &mut meshes, &mut materials, &mut entity_map, pos2);

    entity_map.print_filled_containers();
}

fn update(
    mut commands: Commands,
    mut entity_map: ResMut<EntityMap>,
    mut q: Query<(Entity, &mut Transform, &mut RigidBody)>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds();
    println!("{}", q.iter().len());
    for (e, mut t, mut rb) in &mut q {
        let mut x = t.translation.x;
        let mut y = t.translation.y;

        let vx = rb.velocity.x;
        let vy = rb.velocity.y;
        let ax = rb.acceleration.x;
        let ay = rb.acceleration.y;

        x += vx*dt + (0.5)*ax*dt*dt;
        y += vy*dt + (0.5)*ay*dt*dt;

        t.translation.x = x;
        t.translation.y = y;

        rb.velocity.x += ax*dt;
        rb.velocity.y += ay*dt;
    }

}

fn add_particle(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    entity_map: &mut ResMut<EntityMap>,
    pos: Vec2
) -> u32 {
    let circle = Circle {radius: 5.0};
    let mesh = Mesh2dHandle(meshes.add(circle));
    let color = Color::rgb(1.0, 1.0, 1.0);
    let material = materials.add(color);
    let mesh_component = MaterialMesh2dBundle {
        mesh: mesh,
        material: material,
        ..default()
    };

    let particle_component = Particle::new(
        pos,
        Vec2::new(1.,0.),
        Vec2::new(0.,0.)
    );
    let entity = commands.spawn_empty()
        .insert(mesh_component)
        .insert(particle_component)
        .id();
    let id = entity.index();
    entity_map.add_entity(id, pos);

    return id;
}
