use std::{error::Error};
use macroquad::prelude::*;
use noise::{NoiseFn, Perlin};
use std::collections::HashMap;

pub mod chunk;
use chunk::{Chunk, Block};

pub const CHUNK_SIZE: u32 = 16;
pub const WORLD_HEIGHT: usize = 64;
pub const RENDER_DISTANCE: u32 = 8;

pub struct World {
    pub chunks: HashMap<(i32, i32), Chunk>,
    pub seed: u32,
    perlin: Perlin,
}

impl World {
    pub fn new(seed: u32) -> Self {
        Self { chunks: HashMap::new(), seed, perlin: Perlin::new(seed) }
    }

    // pub fn linearize_pos(pos: [u8; 3]) -> u32 {
    //     let x = pos[0];
    //     let y = pos[1];
    //     let z = pos[2];
    //
    //     let idx = x as u32 + (z as u32 * CHUNK_SIZE) + (y as u32 * CHUNK_SIZE.pow(2));
    //
    //     idx
    // }

    pub fn generate_chunk(&mut self, cx: i32, cz: i32) -> Result<(), Box<dyn Error>> {
        let mut chunk = Chunk::new(cx, cz);
        
        for x in 0..CHUNK_SIZE as u8 {
            for z in 0..CHUNK_SIZE as u8 {
                let wx = (cx * CHUNK_SIZE as i32 + x as i32) as f64;
                let wz = (cz * CHUNK_SIZE as i32 + z as i32) as f64;
                
                let noise_val = self.perlin.get([wx * 0.020, wz * 0.020]);
                let height = ((noise_val + 1.0) * 0.5 * (WORLD_HEIGHT as f64 / 2.0)) as u8;
                // let height = (noise_val * WORLD_HEIGHT as f64) as u32;

                for y in 0..WORLD_HEIGHT as u8 {
                    // let idx = Self::linearize_pos([x, y, z]);
                    chunk.blocks[x as usize][y as usize][z as usize] = 
                    if y == height.saturating_sub(1) {
                        Block(1)
                    } else if y < height {
                        Block(2)
                    } else {
                        Block(0)
                    };
                }
            }
        }
        
        chunk.rebuild_mesh();
        self.chunks.insert((cx, cz), chunk);
        Ok(())
    }

    pub fn load_or_gen(&mut self, cx: i32, cz: i32) {
        if !self.chunks.contains_key(&(cx, cz)) {
            match self.generate_chunk(cx, cz) {
                Ok(()) => {},
                Err(e) => {
                    eprint!("{}", e);
                },
            }
        }
    }
}

/* 
use std::error::Error;

use macroquad::prelude::*;
use noise::{NoiseFn, Perlin};
use std::collections::HashMap;


pub mod chunk;
use chunk::Chunk;

pub const CHUNK_SIZE: i32 = 16; // Using power of 2 is better for performance
pub const WORLD_HEIGHT: usize = 64;

#[derive(Copy, Clone, PartialEq)]
pub enum BlockType {
    Air,
    Dirt,
    Grass,
}

pub struct World {
    pub chunks: HashMap<(i32, i32), Chunk>,
    pub seed: u32,
    perlin: Perlin,
}

impl World {
    pub fn new(seed: u32) -> Self {
        Self {
            chunks: HashMap::new(),
            seed,
            perlin: Perlin::new(seed),
        }
    }

    pub fn generate_chunk(&mut self, cx: i32, cz: i32) -> Result<(), Box<dyn Error>> {
        let mut chunk = Chunk::new();
        
        for x in 0..CHUNK_SIZE as usize {
            for z in 0..CHUNK_SIZE as usize {
                let wx = (cx * CHUNK_SIZE as i32 + x as i32) as f64;
                let wz = (cz * CHUNK_SIZE as i32 + z as i32) as f64;
                
                let noise_val = self.perlin.get([wx * 0.1, wz * 0.1]);
                let height = ((noise_val + 1.0) * 0.5 * (WORLD_HEIGHT as f64 / 2.0)) as usize;

                for y in 0..WORLD_HEIGHT {
                    chunk.blocks[x][y][z] = if y == height.checked_sub(1).ok_or("str")? { 
                        // BlockType::Grass 
                        1
                    } else {
                        // BlockType::Dirt 
                        2
                    };
                }
            }
        }
        chunk.rebuild_mesh((cx, cz));
        self.chunks.insert((cx, cz), chunk);

        return Ok(());
    }

    pub fn load_or_gen(&mut self, cx: i32, cz: i32) {
        if self.chunks.contains_key(&(cx, cz)) {
            return;
        }

        let _ = self.generate_chunk(cx, cz);
    }

    pub fn render(&self) {
        for chunk in self.chunks.values() {
            println!("rendering chunk..");
            if let Some(mesh) = &chunk.mesh {
                println!("mesh drawn.");
                draw_mesh(mesh);
            }
        }
    }
}
*/
