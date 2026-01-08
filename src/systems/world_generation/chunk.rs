use macroquad::prelude::*;
use crate::systems::world_generation::{ CHUNK_SIZE, WORLD_HEIGHT };

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Block(pub u8);

pub struct Chunk {
    pub blocks: [[[Block; CHUNK_SIZE as usize]; WORLD_HEIGHT]; CHUNK_SIZE as usize],
    pub mesh: Option<Mesh>,
    pub dirty: bool,
    origin_pos: Vec3,
}

impl Chunk {
    pub fn new(cx: i32, cz: i32) -> Self {
        let origin_pos = vec3(
            cx as f32 * CHUNK_SIZE as f32,
            0.,
            cz as f32 * CHUNK_SIZE as f32,
        );

        Self {
            blocks: [[[Block(0); CHUNK_SIZE as usize]; WORLD_HEIGHT]; CHUNK_SIZE as usize],
            mesh: None,
            dirty: true,
            origin_pos
        }
    }

    pub fn is_solid(&self, x: i32, y: i32, z: i32) -> bool {
        if x < 0 || z < 0 || 
           x >= CHUNK_SIZE as i32 || 
           z >= CHUNK_SIZE as i32 {
            return true;

        } else if y >= WORLD_HEIGHT as i32 || y < 0 {
            return false;
        };

        self.blocks[x as usize][y as usize][z as usize].0 != 0
    }

    pub fn add_face(&self, block_pos: Vec3, vertices: &[Vec3; 4], color: Color, mesh: &mut Mesh) {
        let base_idx = mesh.vertices.len() as u16;

        //  0 -- 1
        //  |    |
        //  |    |
        //  2 -- 3
        
        for &v_offset in vertices {
            mesh.vertices.push(Vertex::new2(block_pos + v_offset, vec2(0., 0.), color));
        }

        mesh.indices.extend_from_slice(&[
            base_idx + 0, base_idx + 1, base_idx + 2,
            base_idx + 0, base_idx + 2, base_idx + 3,
        ]);
    }

    pub fn rebuild_mesh(&mut self) {
        let mut mesh = Mesh {
            vertices: Vec::new(),
            indices: Vec::new(),
            texture: None,
        };

        for x in 0..CHUNK_SIZE as i32 {
            for y in 0..WORLD_HEIGHT as i32 {
                for z in 0..CHUNK_SIZE as i32 {
                    if !self.is_solid(x, y, z) { continue; }

                    let pos = vec3(x as f32, y as f32, z as f32) + self.origin_pos;
                    
                    // TOP (+Y)
                    if !self.is_solid(x, y + 1, z) {
                        self.add_face(pos, &[
                            vec3(0., 1., 1.), vec3(1., 1., 1.), 
                            vec3(1., 1., 0.), vec3(0., 1., 0.)
                        ], color_u8!(110, 200, 110, 255), &mut mesh);
                    }
                    // BOTTOM (-Y)
                    if !self.is_solid(x, y - 1, z) {
                        self.add_face(pos, &[
                            vec3(0., 0., 0.), vec3(1., 0., 0.), 
                            vec3(1., 0., 1.), vec3(0., 0., 1.)
                        ], color_u8!(100, 70, 40, 255), &mut mesh);
                    }
                    // NORTH (+Z)
                    if !self.is_solid(x, y, z + 1) {
                        self.add_face(pos, &[
                            vec3(1., 0., 1.), vec3(1., 1., 1.), 
                            vec3(0., 1., 1.), vec3(0., 0., 1.)
                        ], color_u8!(90, 180, 90, 255), &mut mesh);
                    }
                    // SOUTH (-Z)
                    if !self.is_solid(x, y, z - 1) {
                        self.add_face(pos, &[
                            vec3(0., 0., 0.), vec3(0., 1., 0.), 
                            vec3(1., 1., 0.), vec3(1., 0., 0.)
                        ], color_u8!(80, 170, 80, 255), &mut mesh);
                    }
                    // EAST (+X)
                    if !self.is_solid(x + 1, y, z) {
                        self.add_face(pos, &[
                            vec3(1., 0., 0.), vec3(1., 1., 0.), 
                            vec3(1., 1., 1.), vec3(1., 0., 1.)
                        ], color_u8!(70, 160, 70, 255), &mut mesh);
                    }
                    // WEST (-X)
                    if !self.is_solid(x - 1, y, z) {
                        self.add_face(pos, &[
                            vec3(0., 0., 1.), vec3(0., 1., 1.), 
                            vec3(0., 1., 0.), vec3(0., 0., 0.)
                        ], color_u8!(70, 160, 70, 255), &mut mesh);
                    }
                }
            }
        }

        self.mesh = Some(mesh);
        self.dirty = false;
    }
}
