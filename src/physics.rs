use bevy::ecs::bundle::DynamicBundle;
use bevy::math::Vec2;
use bevy::prelude::{Commands, Component, Entity, Query, Res, ResMut, Transform, World};
use crate::entity_map::EntityMap;

#[derive(Component)]
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
pub struct CircleCollider {
    radius: f32,
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

    // put entities with their components in a hashmap or something
    // that way we don't need to do world.get within the for loops
    for (
        e,
        t,
        mut rb,
        c
    ) in query.iter_mut(world) {
        let pos = Vec2::new(t.translation.x, t.translation.y);
        let related = entity_map.get_related_entities(pos);
        for other in related {
            let o_t = world.get::<Transform>(other).unwrap().clone();
            let o_c = world.get::<CircleCollider>(other).unwrap().clone();
            let combined_radius = c.radius + o_c.radius;
            if t.translation.distance_squared(o_t.translation) <= combined_radius * combined_radius {
                rb.acceleration = Vec2::new(0.,0.);
                rb.velocity = Vec2::new(0.,0.);
            }
        }
    }
}