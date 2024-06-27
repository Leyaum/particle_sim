use bevy::math::{IVec2, Vec2};
use bevy::ecs::system::Resource;
use bevy::prelude::{Color, Entity, Gizmos, Query, ResMut, Transform};
use crate::particle::Particle;
use crate::physics::RigidBody;

#[derive(Resource)]
pub struct EntityMap {
    map_size: Vec2,
    container_size: f32,

    rows: usize,
    cols: usize,
    containers: Vec<Vec<u32>>,
}

impl EntityMap {
    pub fn new(mut map_size: Vec2, container_size: f32) -> Self {
        if map_size.x < 0.0 {
            map_size.x = -map_size.x;
        } if map_size.y < 0.0 {
            map_size.y = -map_size.y;
        }

        let cols = (map_size.x / container_size).ceil() as usize;
        let rows = (map_size.y / container_size).ceil() as usize;
        let mut containers = Vec::<Vec<u32>>::with_capacity(rows*cols);
        for i in 0..rows*cols {
            let container = Vec::<u32>::new();
            containers.push(container);
        }

        return EntityMap {
            map_size: map_size,
            container_size: container_size,
            rows: rows,
            cols: cols,
            containers: containers,
        }
    }

    pub fn add_entity(&mut self, id: u32, pos: Vec2) {
        let container_index = self.pos_to_container_index(pos);
        let container = &mut self.containers[container_index];
        container.push(id);
    }

    pub fn pos_to_container_index(&mut self, mut pos: Vec2) -> usize {
        let r: usize;
        let c: usize;

        pos.x += self.map_size.x / 2.0;
        pos.y += self.map_size.y / 2.0;

        if pos.x >= self.map_size.x {
            c = self.cols-1;
        } else if pos.x < 0.0 {
            c = 0;
        } else {
            c = (pos.x / self.container_size) as usize;
        }

        if pos.y >= self.map_size.y {
            r = self.rows-1;
        } else if pos.y < 0.0 {
            r = 0;
        } else {
            r = (pos.y / self.container_size) as usize;
        }

        return r*self.cols + c;
    }

    pub fn print_filled_containers(&self) {
        println!();
        for i in (0..self.rows).rev() {
            for j in 0..self.cols {
                let entities = self.containers[i*self.cols + j].len();
                if entities > 0 {
                    print!("{:<3}", entities);
                } else {
                    print!("0  ");
                }
            }
            print!("\n");
        }
    }

    pub fn get_map_size(&self) -> Vec2 {
        return self.map_size;
    }

    pub fn get_map_dims(&self) -> IVec2 {
        return IVec2::new(self.cols as i32, self.rows as i32);
    }

    pub fn get_container_size(&self) -> f32 {
        return self.container_size;
    }
}

pub fn remap(
    mut entity_map: ResMut<EntityMap>,
    q: Query<(Entity, &Transform, &RigidBody)>
) {
    for container in &mut entity_map.containers {
        container.clear();
    }

    for (e, t, rb) in q.iter() {
        let id = e.index();
        let x = t.translation.x;
        let y = t.translation.y;
        let pos = Vec2::new(x, y);
        entity_map.add_entity(id, pos);
    }
}