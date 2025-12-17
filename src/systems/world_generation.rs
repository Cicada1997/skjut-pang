use std::error::Error;

use macroquad::prelude::*;

use noise::{NoiseFn, Perlin, Seedable};


use std::collections::HashMap;

use crate::Game; 
use crate::components::physics::PhysicsObject;
use crate::systems::render::ENTITY_RENDER_DISTANCE;

pub static RENDER_DISTANCE: i32 = 2;
pub static CHUNK_SIZE:      i32 = 24;
pub static WORLD_HEIGHT:    i32 = 64;

pub fn chunk_pos_relative(x: i32, y: i32) -> ((i32, i32), (i32, i32)) {
    let chunk_x = x / CHUNK_SIZE;
    let chunk_y = y / CHUNK_SIZE;
    let chunk_pos = (chunk_x, chunk_y);
    let local_pos = (x - CHUNK_SIZE*chunk_x, y - CHUNK_SIZE*chunk_y);

    return (chunk_pos, local_pos);
}

pub type ChunkPos = (i32, i32);

impl Game {
    pub(crate) fn load_nearby_chunks(&mut self) -> Vec<ChunkPos> {
        let (cnk_x, cnk_y) = {
            let p = self.ecs.get::<&PhysicsObject>(self.player_id).unwrap();
            let (mut p_chunk_pos, _p_chunk_local) = chunk_pos_relative(p.pos.x as i32, p.pos.y as i32);

            p_chunk_pos.0 += RENDER_DISTANCE;
            p_chunk_pos.1 += RENDER_DISTANCE;

            p_chunk_pos
        };

        dbg!(cnk_x);
        dbg!(cnk_y);

        let mut chunks = Vec::new();

        for x in -cnk_x..cnk_x {
            for y in -cnk_y..cnk_y {
                let p = self.ecs.get::<&PhysicsObject>(self.player_id).unwrap();
                if vec2((x + CHUNK_SIZE / 2) as f32, (y + CHUNK_SIZE / 2) as f32).distance(vec2(p.pos.x, p.pos.y)) > ENTITY_RENDER_DISTANCE { continue; }

                let _ = self.world.load_or_gen(x*CHUNK_SIZE, y*CHUNK_SIZE);

                chunks.push((x, y));
            }
        }

        return chunks;
    }

}

// type ChunkPos = (i32, i32);

pub struct World {
    pub chunks: HashMap<(i32, i32), Chunk>,
    pub loaded_chunks: Vec<ChunkPos>,
    pub seed: u32,
}

impl World {
    pub fn new(seed_opt: Option<&str>) -> Self {
        let chunks        = HashMap::new();
        let loaded_chunks = Vec::new();
        let seed: u32     = match seed_opt {
            Some(seed_str) => Self::seed_as_int(seed_str),
            None => rand::gen_range(0, u32::max_value()),
        };

        Self {
            chunks,
            loaded_chunks,
            seed,
        }
    }

    pub fn index(&self, x: i32, y: i32, z: i32) -> Result<usize, Box<dyn Error>> {
        let chunk_x = x / CHUNK_SIZE;
        let chunk_y = y / CHUNK_SIZE;
        // let chunk_pos = (chunk_x, chunk_y);
        let local_pos = (x - CHUNK_SIZE*chunk_x, y - CHUNK_SIZE*chunk_y);

        let index = local_pos.0 + local_pos.1 * CHUNK_SIZE + z * CHUNK_SIZE.pow(2);
        return Ok(index as usize);
        // match self.chunks.get(&chunk_pos) {
        //     Some(_chunk) => {
        //     }
        //
        //     None => return Err("Block is not yet generated!".into()),
        // }
    }

    pub fn fetch_block<'a>(&self, chunk: &'a Chunk, x: i32, y: i32, z: i32) -> Result<&'a Block, Box<dyn Error>> {
        let index = self.index(x, y, z)?;

        match chunk.blocks.get(index) {
            Some(block) => {
                return Ok(block);
            }

            None => return Err("Block is not yet generated!".into()),
        }
    }

    // pub fn fetch_block_mut<'a>(&mut self, chunk: &'a Chunk, x: i32, y: i32, z: i32) -> Result<&'a mut Block, Box<dyn Error>> {
    //     let index = self.index(x, y, z)?;
    //
    //     match chunk.blocks.get_mut(index) {
    //         Some(block) => {
    //             return Ok(block);
    //         }
    //
    //         None => return Err("Block is not yet generated!".into()),
    //     }
    // }

    pub fn index_mut(&mut self, x: i32, y: i32, z: i32) -> Result<&mut Block, Box<dyn Error>> {
        let chunk_x = x / CHUNK_SIZE;
        let chunk_y = y / CHUNK_SIZE;
        let chunk_pos = (chunk_x, chunk_y);
        let local_pos = (x - CHUNK_SIZE*chunk_x, y - CHUNK_SIZE*chunk_y);

        match self.chunks.get_mut(&chunk_pos) {
            Some(chunk) => {

                let index = local_pos.0 + local_pos.1 * CHUNK_SIZE + z * CHUNK_SIZE.pow(2);
                if let Some(block) = chunk.blocks.get_mut(index as usize) {
                    return Ok(block);
                }

                return Err("could not find block".into())
            }

            None => return Err("Block is not yet generated!".into()),
        };

    }

    fn seed_as_int(seed: &str) -> u32 {
        let mut val: u32 = 0;
        for b in seed.as_bytes() {
            val += *b as u32;
        }

        return val;
    }

    fn generate_chunk(&mut self, chunk_x: i32, chunk_y: i32) {
        let perlin = Perlin::new(self.seed);

        warn!("Fix: use mesh!");
        let mut chunk = Chunk { blocks: Vec::new() };

        for x in chunk_x..chunk_x+CHUNK_SIZE {
            for y in 0..WORLD_HEIGHT {
                for z in chunk_y..chunk_y+CHUNK_SIZE {
                    let _val = perlin.get([x as f64, y as f64, z as f64]);

                    let block = Block { color: GREEN, position: vec3( x as f32, y as f32, z as f32) };

                    // let index = self.index(x, y, z).unwrap();

                    chunk.blocks.push(block);
                }
            }
        }

        self.chunks.insert((chunk_x, chunk_y), chunk);
    }

    fn load_or_gen(&mut self, chunk_x: i32, chunk_y: i32) -> Result<&Chunk, Box<dyn Error>> {
        let chunk_pos = (chunk_x, chunk_y);

        if !self.chunks.contains_key(&chunk_pos) {
            info!("generating chunk...");
            self.generate_chunk(chunk_x, chunk_y);
            info!("chunk generated!");
        }

        match self.chunks.get(&chunk_pos) {
            Some(chunk) => {
                Ok(chunk)
            },
            _ => panic!("Unable to generate chunk x: {chunk_x}, y: {chunk_y}")
        }
    }
}

#[derive(Debug, Clone)]
pub struct Chunk {
    pub blocks: Vec<Block>,
    // mesh:   Option<Mesh>
}

impl Chunk {
}

#[derive(Debug, Clone)]
pub struct Block {
    pub color: Color,
    pub position: Vec3,
}
