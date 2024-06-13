use std::collections::HashMap;
use bevy::ecs::system::SystemId;
use bevy::prelude::{FromWorld, Resource, World};

#[derive(Resource)]
struct Systems {
    hash_map: HashMap<String, SystemId>
}

impl FromWorld for Systems {
    fn from_world(word: &mut World) -> Self {
        let systems = Systems {
            hash_map: HashMap::new()
        };

        return systems;
    }
}

impl Systems {
    pub fn add_system(&mut self,) {

    }
}

enum OneShots {

}