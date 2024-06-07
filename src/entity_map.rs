use bevy::math::{IVec2, Vec2};

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
        println!("num containers: {}", containers.len());

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
        println!("entity index: {}", container_index);
        let container = &mut self.containers[container_index];
        container.push(id);

        for i in (0..self.rows).rev() {
            for j in 0..self.cols {
                if i*self.cols + j == container_index {
                    print!("X ");
                } else {
                    print!("0 ");
                }
            }
            print!("\n");
        }
    }

    pub fn pos_to_container_index(&mut self, mut pos: Vec2) -> usize {
        pos.x += self.map_size.x / 2.0;
        pos.y += self.map_size.y / 2.0;
        if pos.x > self.map_size.x {
            pos.x = self.map_size.x;
        } else if pos.x < 0.0 {
            pos.x = 0.0;
        }
        if pos.y > self.map_size.y {
            pos.y = self.map_size.y;
        } else if pos.y < 0.0 {
            pos.y = 0.0;
        }

        let r = (pos.y / self.container_size) as usize;
        let c = (pos.x / self.container_size) as usize;
        return r*self.cols + c;
    }
}