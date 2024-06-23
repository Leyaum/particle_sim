use std::collections::HashMap;
use bevy::math::Vec2;
use bevy::prelude::{Component, Entity, Query, Res, Time, Transform, World};
use crate::entity_map::EntityMap;
use crate::math_helpers::{vector_magnitude, vector_project};

#[derive(Component)]
#[derive(Clone)]
pub struct RigidBody {
    pub mass: f32,
    pub velocity: Vec2,
    pub acceleration: Vec2,
}

impl Default for RigidBody {
    fn default() -> Self {
        return RigidBody {
            mass: 1.0,
            velocity: Vec2::new(0.0,0.0),
            acceleration: Vec2::new(0.0,0.0),
        }
    }
}

impl RigidBody {
    pub fn new(mass: f32, velocity: Vec2, acceleration: Vec2) -> Self {
        return RigidBody {
            mass: mass,
            velocity: velocity,
            acceleration: acceleration,
        }
    }
}

#[derive(Component)]
#[derive(Clone)]
pub struct CircleCollider {
    pub radius: f32,
}

impl Default for CircleCollider {
    fn default() -> Self {
        return CircleCollider {
            radius: 1.0,
        }
    }
}

pub fn update_rigid_bodies(
    mut q: Query<(Entity, &mut Transform, &mut RigidBody)>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds();
    for (e, mut t, mut rb) in &mut q {
        let vx = rb.velocity.x;
        let vy = rb.velocity.y;
        let ax = rb.acceleration.x;
        let ay = rb.acceleration.y;

        t.translation.x += vx*dt + (0.5)*ax*dt*dt;
        t.translation.y += vy*dt + (0.5)*ay*dt*dt;

        rb.velocity.x += ax*dt;
        rb.velocity.y += ay*dt;
    }
}

pub fn resolve_collisions(
    world: &mut World,
) {
    if !world.contains_resource::<EntityMap>() {
        return;
    }
    let entity_map = world.get_resource::<EntityMap>().unwrap().clone();

    let mut query = world.query::<(
        Entity,
        &Transform,
        &mut RigidBody,
        &CircleCollider
    )>();

    let mut entities = Vec::<Entity>::new();
    let mut comp_map = HashMap::<Entity,(Transform, RigidBody, CircleCollider)>::new();

    let mut tot_ke = 0.0;
    let mut tot_lm = Vec2::new(0.,0.);
    for (
        e,
        t,
        rb,
        c
    ) in query.iter(world) {
        comp_map.insert(e,(t.clone(), rb.clone(), c.clone()));
        entities.push(e);
        tot_ke += 0.5 * rb.mass * rb.velocity.length_squared();
        tot_lm += rb.mass * rb.velocity;
    }
    println!("Kinetic Energy: {tot_ke}, Linear Momentum: {tot_lm}");

    for (
        e,
        t,
        mut rb,
        c
    ) in query.iter_mut(world) {
        /*
            TODO: Precision improvement:
            Detect collision with raycast to make physics less dependent on discrete timesteps
            This means figuring out exactly when within the last timestep collision occured
            Recalculate velocity and position at that point in time
            Then do the remaining calculations
        */

        /*
            TODO: Collision optimization:
            Skip over particles that aren't moving
            If another particle bumps into a non-moving one,
            add it to a list of particles to return to and calculate the new trajectory
        */

        let pos = Vec2::new(t.translation.x, t.translation.y);
        let related = entity_map.get_related_entities(pos);
        for other in related {
            if other.index() == e.index() {
                continue;
            }
            let (o_t,o_rb,o_c) = comp_map.get(&other).unwrap();
            let combined_radius = c.radius + o_c.radius;
            if t.translation.distance_squared(o_t.translation) <= combined_radius*combined_radius {
                let displacement = t.translation.truncate() - o_t.translation.truncate();
                rb.velocity = calculate_collision_trajectory(
                    rb.velocity,
                    o_rb.velocity,
                    rb.mass,
                    o_rb.mass,
                    displacement,
                );
            }
        }
    }
}

fn calculate_collision_trajectory(
    vel_1: Vec2,
    mut vel_2: Vec2,
    m1: f32,
    m2: f32,
    displacement: Vec2,
) -> Vec2 {
    // Change reference frame to make object 1 at rest
    vel_2 -= vel_1;

    vel_2 = vector_project(vel_2, displacement);

    // Change reference frame so collision happens on 1 dimension
    let v2 = vector_magnitude(vel_2);

    let v1_new = 2.0*m2*v2/(m1+m2);

    // Revert reference frame
    let trajectory = vel_2.normalize_or_zero() * v1_new + vel_1;

    return trajectory;
}