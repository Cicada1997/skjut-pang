use macroquad::prelude::*;

use noise::{NoiseFn, Perlin, Seedable};


use std::collections::HashMap;

// use crate::Game;
//
type ChunkPos = (i32, i32);

struct World {
    chunks_map: HashMap<ChunkPos, Chunk>,
    loaded_chunks: Vec<Chunk>,
    seed: String,
}

impl World {
    fn seed_as_int(&self) -> u32 {
        let mut val: u32 = 0;
        for b in self.seed.clone().as_bytes() {
            val += *b as u32;
        }

        return val;
    }

    fn generate_chunk(&mut self, chunk_x: i32, chunk_y: i32) {
        let perlin = Perlin::new(self.seed_as_int());

        warn!("Fix: use mesh!");
        let chunk = Chunk { blocks: Vec::new() };

        for x in chunk_x..chunk_x+24 {
            for y in 0..64 {
                for z in chunk_y..chunk_y+24 {
                    let val = perlin.get([x as f64, y as f64, z as f64]);

                    if val > 0.6 {
                        let block = Block { color: GREEN, position: vec3( x as f32, y as f32, z as f32) };

                        let index = chunk.blocks.len();
                    }
                }
            }
        }

        self.chunks_map.insert((chunk_x, chunk_y), chunk);
    }

    fn load_or_gen(&mut self, chunk_x: i32, chunk_y: i32) -> &Chunk {
        if let Some(chunk) = self.chunks_map.get(&(chunk_x, chunk_y)) {
            return chunk;
        };

        self.generate_chunk(chunk_x, chunk_y);
        match self.chunks_map.get(&(chunk_x, chunk_y)) {
            Some(chunk) => return chunk,
            _ => panic!("Unable to generate chunk x: {chunk_x}, y: {chunk_y}")
        }
    }
}

#[derive(Debug, Clone)]
struct Chunk {
    blocks: Vec<Block>,
    // mesh:   Option<Mesh>
}

impl Chunk {
}

#[derive(Debug, Clone)]
struct Block {
    color: Color,
    position: Vec3,
}
