use bevy::{
    prelude::*,
    sprite::Material2d,
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