use luola::constants;
use luola::creature::{creature_types, Creature};
use luola::world::*;
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;

pub fn generate_layer(layer_i: i32, rng: &mut ChaCha20Rng) -> Layer {
    let mut layer = Layer::new(constants::WORLD_HEIGHT, constants::WORLD_WIDTH);

    for i in 0..layer.height() {
        for j in 0..layer.width() {
            let is_wall: bool = rng.gen_range(0..10) <= layer_i;

            if is_wall {
                let square = GridSquare { y: i, x: j };
                layer.set_tile(square, Tile::Wall);
            }
        }
    }

    for _ in 0..(5 * (layer_i + 1)) {
        let mut creature: Creature = creature_types::create_testcreature();
        let pos = GridSquare {
            y: rng.gen_range(0..constants::WORLD_HEIGHT),
            x: rng.gen_range(0..constants::WORLD_WIDTH),
        };
        creature.set_position(&pos);

        layer.add_creature(creature);
    }

    layer
}

pub fn generate_world(rng_seed: u64) -> World {
    let mut rng = ChaCha20Rng::seed_from_u64(rng_seed);

    let mut world = World::new();

    for i in 0..constants::WORLD_LAYERS {
        let layer = generate_layer(i, &mut rng);
        world.layers.push(layer);
    }

    world
}
