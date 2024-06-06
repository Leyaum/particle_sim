use bevy::prelude::{Camera2d, Camera2dBundle};

pub struct ParticleWorld {
    pub camera: Camera2d,
}

impl Default for ParticleWorld {
    fn default() -> Self {
        return ParticleWorld {
            camera: Camera2dBundle::default().camera_2d,
        };
    }
}

impl ParticleWorld {
    pub fn test(&mut self) -> String {
        return String::from("test");
    }
}