use luola::ai::AI;
use luola::constants;
use luola::creature::{creature_types, Creature};
use luola::grid::{GridSquare, Tile};
use luola::world::{Entity, Layer, World};
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;

pub fn generate_layer(layer_i: i32, rng: &mut ChaCha20Rng) -> Layer {
    let mut layer = Layer::new(constants::WORLD_HEIGHT, constants::WORLD_WIDTH);

    for i in 0..layer.grid.height() {
        for j in 0..layer.grid.width() {
            let is_wall: bool = rng.gen_range(0..10) <= layer_i;

            if is_wall {
                let square = GridSquare { y: i, x: j };
                layer.grid.set_tile(square, Tile::Wall);
            }
        }
    }

    for i in 0..(5 * (layer_i + 1)) {
        let pos = GridSquare {
            y: rng.gen_range(0..constants::WORLD_HEIGHT),
            x: rng.gen_range(0..constants::WORLD_WIDTH),
        };
        let creature: Creature = creature_types::create_testcreature(5 * i, pos);

        let c_ai: AI = AI::new(creature.get_id());

        layer.add_creature(creature, c_ai);
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
