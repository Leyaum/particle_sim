use std::default::Default;
use bevy::{
    prelude::*,
};
use crate::rigid_body::RigidBody;

#[derive(Bundle)]
pub struct Particle {
    pub transform: Transform,
    pub rigidbody: RigidBody,
}

impl Default for Particle {
    fn default() -> Self {
        Self {
            transform: Default::default(),
            rigidbody: Default::default(),
        }
    }
}

impl Particle {
    pub fn new(pos: Vec2) -> Self {
        let transform = Transform::from_xyz(pos.x, pos.y, 0.0);
        Self {
            transform: transform,
            rigidbody: RigidBody::default(),
        }
    }
}