mod particle_world;
mod sim_state;
mod particle;
mod entity_map;
mod physics;
mod systems;
mod math_helpers;
mod debug;

use crate::physics::{
    update_rigid_bodies,
    resolve_particle_collisions,
    resolve_wall_collisions
};
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use rand::random;
use crate::entity_map::*;
use crate::particle::*;
use crate::debug::*;

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
            update_rigid_bodies.after(update),
            resolve_particle_collisions.after(update_rigid_bodies),
            resolve_wall_collisions.after(resolve_particle_collisions),
            remap.after(resolve_particle_collisions)
        ))
        .add_systems(Update, (
            draw_gizmos,
            write_debug_physics,
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

    let pos1 = Vec2::new(-200.0, 0.0);
    add_particle(
        &mut commands,
        &mut meshes,
        &mut materials,
        &mut entity_map,
        pos1,
        10.0,
        Vec2::new(800.0,0.0),
        Vec2::new(0.0,0.0),
    );

    let pos2 = Vec2::new(200.0, 25.0);
    add_particle(
        &mut commands,
        &mut meshes,
        &mut materials,
        &mut entity_map,
        pos2,
        10.0,
        Vec2::new(-800.0, 0.0),
        Vec2::new(0.0, 0.0)
    );

    //entity_map.print_filled_containers();

    let debug_text_style = TextStyle {
        font: Default::default(),
        font_size: 20.0,
        color: Default::default(),
    };
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Kinetic Energy: ",
                debug_text_style.clone()
            ),
            TextSection::from_style(debug_text_style.clone()),
            TextSection::new(
                "Linear Momentum: ",
                debug_text_style.clone()
            ),
            TextSection::from_style(debug_text_style.clone())
        ]),
        DebugPhysicsText,
    ));
}

fn update(
    mut commands: Commands,
    mut entity_map: ResMut<EntityMap>,
) {
    //entity_map.print_filled_containers();
}

fn add_particle(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    entity_map: &mut ResMut<EntityMap>,
    pos: Vec2,
    mass: f32,
    velocity: Vec2,
    acceleration: Vec2,
) -> Entity {
    let particle_size = 20.0;

    let circle = Circle {radius: particle_size};
    let mesh = Mesh2dHandle(meshes.add(circle));
    let color = Color::rgb(random::<f32>(), random::<f32>(), random::<f32>());
    let material = materials.add(color);
    let mesh_component = MaterialMesh2dBundle {
        mesh: mesh,
        material: material,
        ..default()
    };

    let particle_component = Particle::new(pos, mass, velocity, acceleration, particle_size);
    let entity = commands.spawn_empty()
        .insert(mesh_component)
        .insert(particle_component)
        .id();
    entity_map.add_entity(entity, pos);

    return entity;
}
