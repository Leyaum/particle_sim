use bevy::{
    prelude::*
};
use bevy::prelude::Transform;

#[derive(Component)]
pub struct Particle {
    pub transform: Transform,
    pub velocity: Vec2,
    pub acceleration: Vec2,
}

impl Particle {
    pub const IDENTITY: Self
}

impl Default for Particle {
    fn default() -> Self {
        return Particle {
            transform: Transform::default(),
            velocity: Vec2::new(0.0,0.0),
            acceleration: Vec2::new(0.0,0.0),
        }
    }
}