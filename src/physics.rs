use bevy::ecs::bundle::DynamicBundle;
use bevy::math::Vec2;
use bevy::prelude::{Commands, Component, Entity, Query, ResMut, Transform, World};
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
    mut commands: Commands,
    world: &mut World,
    mut q: Query<(Entity, &Transform, &mut RigidBody, &CircleCollider)>,
) {
    if !world.contains_resource::<EntityMap>() {
        return;
    }

    let entity_map = world.get_resource::<EntityMap>().unwrap();

    // Collect entities and their data
    let entities: Vec<(Entity, Vec2, f32)> = q.iter()
        .map(|(e, t, _, c)| (e, Vec2::new(t.translation.x, t.translation.y), c.radius))
        .collect();

    for (e, t, mut rb, c) in q.iter_mut() {
        let pos = Vec2::new(t.translation.x, t.translation.y);
        let related = entity_map.get_related_entities(pos);

        for other in related {
            if other == e {
                continue;
            }

            // Use a separate scope to ensure borrows from `world` don't overlap
            let rel_t = world.get::<Transform>(other).unwrap();
            let rel_c = world.get::<CircleCollider>(other).unwrap();
            let related_pos = Vec2::new(rel_t.translation.x, rel_t.translation.y);
            let combined_radius = c.radius + rel_c.radius;
            if pos.distance_squared(related_pos) < combined_radius * combined_radius {
                rb.acceleration = Vec2::new(0.0, 0.0);
                rb.velocity = Vec2::new(0.0, 0.0);
            }
        }
    }
}