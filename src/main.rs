mod creature;
mod map;
mod neural_network;
mod visual;

use std::sync::{Arc, Mutex};

fn main() {
    let creature_factory = creature::CreatureFactory::new();

    let creatures = Arc::new(Mutex::new(creature_factory.create_multiple_random(100)));

    let map = map::Map::new(100, 100, &creatures);



    for creature in creatures.lock().unwrap().iter_mut() {
        creature.update(1.0);
    }

    map.get_creatures();


    visual::start(&map);
}
