use bevy::math::Vec2;

pub fn vector_magnitude(v: Vec2) -> f32 {
    return f32::sqrt(v.x*v.x + v.y*v.y);
}
/// Returns v1 projected onto v2
pub fn vector_project(v1: Vec2, v2: Vec2) -> Vec2 {
    return vector_magnitude(v1) * v2.normalize();
}