use std::collections::HashMap;
use bevy::math::Vec2;
use bevy::prelude::{Component, Entity, Query, Res, Time, Transform, World};
use crate::entity_map::EntityMap;
use crate::math_helpers::{quadratic_formula, vector_project};

/*
    TODO: Swap all physics related calculations to 64 bit floating point numbers
    We will keep track of position with our own transform component
    Update the default transform only before rendering
*/

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

    // returns position of particle using its current trajectory
    pub fn calc_delta_position(&self, t: f32) -> Vec2 {
        let mut delta = Vec2::new(0.,0.);
        delta.x = self.velocity.x * t + 0.5 * self.acceleration.x * t*t;
        delta.y = self.velocity.y * t + 0.5 * self.acceleration.y * t*t;
        return delta;
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

// update rigid body positions using verlet integration
pub fn update_rigid_bodies(
    mut q: Query<(Entity, &mut Transform, &mut RigidBody)>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds() * 1.0;
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

// makes particles bounce off walls
pub fn resolve_wall_collisions(
    entity_map: Res<EntityMap>,
    mut q: Query<(&Transform, &mut RigidBody, &CircleCollider)>
) {
    // TODO: Calculate collisions with raycasts to be more precise
    // TODO: Get only entities that are on edge containers in the entity_map
    // TODO: Change this to use some sort of constraints system
    for (t, mut rb, c) in q.iter_mut() {
        let x_size = entity_map.get_map_size().x / 2.0;
        let y_size = entity_map.get_map_size().y / 2.0;

        if t.translation.x - c.radius <= -x_size && rb.velocity.x < 0.0 {
            rb.velocity.x *= -1.0;
        } else if t.translation.x + c.radius >= x_size && rb.velocity.x > 0.0  {
            rb.velocity.x *= -1.0;
        }

        if t.translation.y - c.radius <= -y_size && rb.velocity.y < 0.0 {
            rb.velocity.y *= -1.0;
        } else if t.translation.y + c.radius >= y_size && rb.velocity.y > 0.0  {
            rb.velocity.y *= -1.0;
        }
    }
}

pub fn resolve_particle_collisions(
    world: &mut World,
) {
    if !world.contains_resource::<EntityMap>() {
        return;
    }
    if !world.contains_resource::<Time>() {
        return;
    }

    let entity_map = world.get_resource::<EntityMap>().unwrap().clone();
    let time = world.get_resource::<Time>().unwrap().clone();

    let mut query = world.query::<(
        Entity,
        &mut Transform,
        &mut RigidBody,
        &CircleCollider
    )>();

    let mut entities = Vec::<Entity>::new();
    let mut comp_map = HashMap::<Entity,(Transform, RigidBody, CircleCollider)>::new();

    // comp_map is needed to allow the next for loop to reference data from the other particles
    for (e, t, rb, c) in query.iter(world) {
        comp_map.insert(e,(t.clone(), rb.clone(), c.clone()));
        entities.push(e);
    }

    for (e, mut t, mut rb, c) in query.iter_mut(world) {
        /*
            TODO: Precision improvement:
            Detect collision with raycast to make physics less dependent on discrete timesteps
            This means figuring out exactly when within the last timestep collision occurred
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
        if related.len() <= 1 {
            continue;
        }

        let mut total_additive_vel = Vec2::new(0.,0.);
        let mut total_additive_pos = Vec2::new(0.,0.);
        for other in related {
            if other.index() == e.index() {
                continue;
            }
            let (o_t,o_rb,o_c) = comp_map.get(&other).unwrap();
            let combined_radius = c.radius + o_c.radius;

            // if colliding
            if t.translation.distance_squared(o_t.translation) < combined_radius*combined_radius {
                let collision_time = calculate_exact_collision_time(
                    t.translation.truncate(),
                    o_t.translation.truncate(),
                    rb.clone(),
                    o_rb.clone(),
                    combined_radius
                );
                let precise_collision_pos = rb.calc_delta_position(collision_time);
                total_additive_pos += precise_collision_pos;

                let displacement = t.translation.truncate() - o_t.translation.truncate();
                let additive_vel = calculate_additive_collision_trajectory(
                    rb.velocity,
                    o_rb.velocity,
                    rb.mass,
                    o_rb.mass,
                    displacement,
                );
                total_additive_vel += additive_vel;

                // attempt to move particles to where they should be assuming they collided at the right moment
                let dt = time.delta_seconds() + collision_time;
                //total_additive_pos.x += additive_vel.x*dt + (0.5)*rb.acceleration.x*dt*dt;
                //total_additive_pos.y += additive_vel.y*dt + (0.5)*rb.acceleration.y*dt*dt;
            }
        }
        t.translation.x += total_additive_pos.x;
        t.translation.y += total_additive_pos.y;
        rb.velocity += total_additive_vel;
    }
}

fn calculate_additive_collision_trajectory(
    vel_1: Vec2,
    mut vel_2: Vec2,
    m1: f32,
    m2: f32,
    displacement: Vec2,
) -> Vec2 {
    // Change reference frame to make object 1 at rest
    vel_2 -= vel_1;

    let v1_new = vector_project(vel_2, displacement) * 2.0*m2/(m1+m2);
    return v1_new;
}

// attempts to find the point of intersection between the trajectories of two particles
fn calculate_exact_collision_time(
    pos1: Vec2,
    pos2: Vec2,
    rb1: RigidBody,
    rb2: RigidBody,
    combined_radius: f32,
) -> f32 {
    // TODO: Make this function take acceleration into account
    // TODO: Fix problem with time coming out positive sometimes
    // I think the particles sometimes overlap too much and it causes the radius vector to be pointing
    // opposite where you would intuitively think it would point
    // this causes the rest of the calculations to be wrong
    let radius_vector = (pos2 - pos1).normalize_or_zero() * combined_radius;

    let x_t = (radius_vector.x - pos2.x + pos1.x) / (rb2.velocity.x - rb1.velocity.x);
    let y_t = (radius_vector.y - pos2.y + pos1.y) / (rb2.velocity.y - rb1.velocity.y);
    if (x_t.is_nan() && y_t.is_nan()) {
        return 0.0;
    }
    if x_t == y_t {
        println!("collision time: {x_t}");
        return x_t;
    }
    if x_t < 0.0 && y_t.is_nan() {
        println!("collision time: {x_t}");
        return x_t;
    }
    if y_t < 0.0 && x_t.is_nan() {
        println!("collision time: {y_t}");
        return y_t;
    }
    println!("what??? {x_t}, {y_t}");
    return 0.0;
}

fn negative_or_zero(num: f32) -> f32 {
    if num < 0.0 {
        return num;
    }
    return -num;
}