use bevy::math::Vec2;
use bevy::prelude::{Color, Component, Gizmos, Query, ResMut};
use bevy::text::Text;
use crate::entity_map::EntityMap;
use crate::physics::RigidBody;

pub fn draw_gizmos(
    mut gizmos: Gizmos,
    entity_map: ResMut<EntityMap>,
) {
    let half_x = entity_map.get_map_size().x/2.0;
    let half_y = entity_map.get_map_size().y/2.0;

    let bot_left = Vec2::new(-half_x, -half_y);
    let top_left = Vec2::new(-half_x, half_y);
    let bot_right = Vec2::new(half_x, -half_y);

    let color = Color::rgb(0.,0.,0.);

    let dims = entity_map.get_map_dims();
    let container_size = entity_map.get_container_size();

    let mut offset = Vec2::new(0.,0.);
    for i in 0..=dims.x {
        gizmos.line_2d(
            bot_left + offset,
            top_left + offset,
            color
        );
        offset += Vec2::new(container_size, 0.);
    }

    offset = Vec2::new(0.,0.);
    for i in 0..=dims.y {
        gizmos.line_2d(
            bot_left + offset,
            bot_right + offset,
            color
        );
        offset += Vec2::new(0., container_size);
    }
}

#[derive(Component)]
pub struct DebugPhysicsText;

pub fn write_debug_physics(
    q_rb: Query<&RigidBody>,
    mut q_text: Query<(&mut Text, &DebugPhysicsText)>
) {
    let mut tot_ke = 0.0;
    let mut tot_lm = Vec2::default();
    for rb in q_rb.iter() {
        tot_ke += 0.5 * rb.mass * rb.velocity.length_squared();
        tot_lm += rb.mass * rb.velocity;
    }

    for (mut text, _) in q_text.iter_mut() {
        text.sections[1].value = format!("{tot_ke}\n");
        text.sections[3].value = format!("{tot_lm}\n");
    }
}