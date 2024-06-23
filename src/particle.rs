use std::default::Default;
use bevy::{
    prelude::*,
};
use crate::physics::{CircleCollider, RigidBody};

#[derive(Bundle)]
pub struct Particle {
    pub transform: Transform,
    pub rigidbody: RigidBody,
    pub circle_collider: CircleCollider,
}

impl Default for Particle {
    fn default() -> Self {
        Self {
            transform: Default::default(),
            rigidbody: Default::default(),
            circle_collider: Default::default(),
        }
    }
}

impl Particle {
    pub fn new(pos: Vec2, mass: f32, velocity: Vec2, acceleration: Vec2, radius: f32) -> Self {
        let transform = Transform::from_xyz(pos.x, pos.y, 0.0);
        Self {
            transform: transform,
            rigidbody: RigidBody {
                mass: mass,
                velocity: velocity,
                acceleration: acceleration,
            },
            circle_collider: CircleCollider {
                radius: radius,
            }
        }
    }
}