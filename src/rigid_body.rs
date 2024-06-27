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