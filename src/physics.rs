use std::collections::HashMap;
use bevy::ecs::bundle::DynamicBundle;
use bevy::math::Vec2;
use bevy::prelude::{Circle, Commands, Component, Entity, Query, Res, ResMut, Transform, World};
use crate::entity_map::EntityMap;

#[derive(Component)]
#[derive(Clone)]
pub struct RigidBody {
    pub velocity: Vec2,
    pub acceleration: Vec2,
}

impl Default for RigidBody {
    fn default() -> Self {
        return RigidBody {
            velocity: Vec2::new(0.0,0.0),
            acceleration: Vec2::new(0.0,0.0),
        }
    }
}

impl RigidBody {
    pub fn new(velocity: Vec2, acceleration: Vec2) -> Self {
        return RigidBody {
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

pub fn calculate_collisions(
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

    for (
        e,
        t,
        rb,
        c
    ) in query.iter(world) {
        comp_map.insert(e,(t.clone(), rb.clone(), c.clone()));
        entities.push(e);
    }

    for (
        e,
        t,
        mut rb,
        c
    ) in query.iter_mut(world) {
        let pos = Vec2::new(t.translation.x, t.translation.y);
        let related = entity_map.get_related_entities(pos);
        for other in related {
            if (other.index() == e.index()) {
                continue;
            }
            let (o_t,_,o_c) = comp_map.get(&other).unwrap();
            let combined_radius = c.radius + o_c.radius;
            println!("combined radius: {}", combined_radius);
            println!("total distance: {}", t.translation.distance_squared(o_t.translation));
            if t.translation.distance_squared(o_t.translation) <= combined_radius*combined_radius {
                rb.acceleration = Vec2::new(0.,0.);
                rb.velocity = Vec2::new(0.,0.);
            }
        }
    }
}