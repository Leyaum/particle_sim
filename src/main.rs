mod particle_world;
mod sim_state;
mod particle;
mod entity_map;
mod physics;
mod systems;

use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use crate::entity_map::{EntityMap, remap};
use crate::particle::Particle;
use crate::physics::RigidBody;

fn main() {
    let map_size = Vec2::new(500.0,500.0);
    let map = EntityMap::new(map_size, 50.0);

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_systems(Startup, (
            setup,
        ))
        .add_systems(Update, (
            update,
            remap.after(update),
            draw_gizmos,
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
    add_particle(
        &mut commands,
        &mut meshes,
        &mut materials,
        &mut entity_map,
        pos1,
        Vec2::new(10.0,10.0),
        Vec2::new(100.0,100.0),
    );

    let pos2 = Vec2::new(-25.0, -25.0);
    add_particle(
        &mut commands,
        &mut meshes,
        &mut materials,
        &mut entity_map,
        pos2,
        Vec2::new(-500.0, 10.0),
        Vec2::new(100.0, 0.0)
    );

    entity_map.print_filled_containers();
}

fn update(
    mut commands: Commands,
    mut entity_map: ResMut<EntityMap>,
    mut q: Query<(Entity, &mut Transform, &mut RigidBody)>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds();
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
    pos: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
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

    let particle_component = Particle::new(pos, velocity, acceleration);
    let entity = commands.spawn_empty()
        .insert(mesh_component)
        .insert(particle_component)
        .id();
    let id = entity.index();
    entity_map.add_entity(id, pos);

    return id;
}

fn draw_gizmos(
    mut gizmos: Gizmos,
    mut entity_map: ResMut<EntityMap>,
) {
    let half_x = entity_map.get_map_size().x/2.0;
    let half_y = entity_map.get_map_size().y/2.0;

    let bot_left = Vec2::new(-half_x, -half_y);
    let top_left = Vec2::new(-half_x, half_y);
    let bot_right = Vec2::new(half_x, -half_y);

    let color = Color::rgb(0.,0.,0.);

    let dims = entity_map.get_map_dims();
    let container_size = entity_map.get_container_size();

    let mut offset = Vec2::new(0.,0.);
    for i in 0..=dims.x {
        gizmos.line_2d(
            bot_left + offset,
            top_left + offset,
            color
        );
        offset += Vec2::new(container_size, 0.);
    }

    offset = Vec2::new(0.,0.);
    for i in 0..=dims.y {
        gizmos.line_2d(
            bot_left + offset,
            bot_right + offset,
            color
        );
        offset += Vec2::new(0., container_size);
    }
}
