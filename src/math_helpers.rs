use bevy::math::{Vec2};

pub fn vector_magnitude(v: Vec2) -> f32 {
    return f32::sqrt(v.x*v.x + v.y*v.y);
}

pub fn vector_dot_product(v1: Vec2, v2: Vec2) -> f32 {
    return v1.x*v2.x + v1.y*v2.y;
}

/// Returns v1 projected onto v2
pub fn vector_project(v1: Vec2, v2: Vec2) -> Vec2 {
    return vector_dot_product(v1, v2) / v2.length_squared() * v2;
}