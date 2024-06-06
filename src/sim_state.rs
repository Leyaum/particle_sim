use bevy::prelude::Resource;

#[derive(Resource)]
pub struct WindowState {
    pub width: f32,
    pub height: f32
}

