use bevy::math::Vec2;
use bevy::prelude::Component;

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