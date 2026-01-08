

// use macroquad::prelude::*;
//
// use crate::systems::world_generation::{ CHUNK_SIZE, WORLD_HEIGHT, chunk };
// // use block_mesh::ndshape::{ConstShape, ConstShape3u32};
// // use block_mesh::{
// //     greedy_quads, GreedyQuadsBuffer, RIGHT_HANDED_Y_UP_CONFIG, 
// //     Voxel, VoxelVisibility, MergeVoxel
// // };
//
// // 16x64x16 with 1-block padding on all sides
// // pub type PaddedChunkShape = ConstShape3u32<16, 64, 16>;
//
// #[derive(Copy, Clone, PartialEq, Eq)]
// pub struct Block(pub u8);
//
// // impl Voxel for Block {
// //     fn get_visibility(&self) -> VoxelVisibility {
// //         if self.0 == 0 { VoxelVisibility::Empty } else { VoxelVisibility::Opaque }
// //     }
// // }
// //
// // impl MergeVoxel for Block {
// //     type MergeValue = u8;
// //     fn merge_value(&self) -> Self::MergeValue { self.0 }
// // }
//
// pub struct Chunk {
//     // pub blocks: Box<[Block; PaddedChunkShape::SIZE as usize]>,
//     pub blocks: [[[Block; 16]; 64]; 16],
//     pub mesh: Option<Mesh>,
//     pub dirty: bool,
//
//     origin_pos: Vec3,
// }
//
// pub enum Dir {
//     Up,
//     Down,
//
//     North,
//     South,
//     West,
//     East,
// }
//
// impl Chunk {
//     pub fn new(cx: i32, cz: i32) -> Self {
//         let origin_pos = vec3(
//             cx as f32 * CHUNK_SIZE as f32,
//             0.,
//             cz as f32 * CHUNK_SIZE as f32,
//         );
//
//         Self {
//             // blocks: Box::new([Block(0); PaddedChunkShape::SIZE as usize]),
//             blocks: [[[Block(0); 16]; 64]; 16],
//             mesh: None,
//             dirty: true,
//
//             origin_pos
//         }
//     }
//
//     pub fn is_solid(&self, x: i16, y: i16, z: i16) -> bool {
//         // if at chunk border:
//         if (x < 0) | (y < 0) | (z < 0) |
//            (x >= CHUNK_SIZE as i16) | 
//            (y >= WORLD_HEIGHT as i16) | 
//            (z >= CHUNK_SIZE as i16) {
//                     return false;
//         }
//
//         let (x, y, z) = (
//             // x.clamp(0, CHUNK_SIZE   as i16 -1) as usize,
//             // y.clamp(0, WORLD_HEIGHT as i16 -1) as usize,
//             // z.clamp(0, CHUNK_SIZE   as i16 -1) as usize,
//             x as usize,
//             y as usize,
//             z as usize,
//         );
//
//         return self.blocks[x][y][z].0 != 0;
//     }
//
//     pub fn add_face(&mut self, pos: Vec3, dots: &[Vec3; 4], color: Color, mesh: &mut Mesh) {
//         let idx = mesh.vertices.len() as u16;
//         for dot in dots.iter() {
//             mesh.vertices.push(Vertex::new2(pos + *dot, vec2(0., 0.), color));
//         }
//
//         //  0 -- 1
//         //  |    |
//         //  |    |
//         //  2 -- 3
//
//         mesh.indices.extend_from_slice(&[
//             idx+0, idx+3, idx+2,
//             idx+0, idx+1, idx+3,
//         ]);
//
//     }
//
//     pub fn build_faces(&mut self, mesh: &mut Mesh) {
//         for x in 0..(CHUNK_SIZE) as i16 {
//             for y in 0..(WORLD_HEIGHT) as i16 {
//                 for z  in 0..(CHUNK_SIZE) as i16 {
//                     // println!("x: {}; y: {}; z: {}", &x, &y, &z);
//                     if !self.is_solid(x, y, z) { continue; }
//
//                     let pos = vec3(x as f32, y as f32, z as f32) + self.origin_pos;
//
//                     // TOP
//                     if !self.is_solid(x, y + 1, z) {
//                         self.add_face(
//                             pos,
//                             &[
//                             vec3(0., 1., 0.), vec3(1., 1., 0.),
//                             vec3(0., 1., 1.), vec3(1., 1., 1.),
//                             ],
//                             color_u8!(0, 255, 0, 255),
//                             mesh
//                         );
//                     }
//                     // DOWN
//                     if !self.is_solid(x, y-1, z) {
//                         self.add_face(
//                             pos,
//                             &[
//                             vec3(0., 0., 0.), vec3(1., 0., 0.),
//                             vec3(0., 0., 1.), vec3(1., 0., 1.),
//                             ],
//                             color_u8!(0, 100, 0, 255),
//                             mesh
//                         );
//                     }
//
//                     // NORTH
//                     if !self.is_solid(x, y, z + 1) {
//                         self.add_face(
//                             pos,
//                             &[
//                             vec3(0., 0., 1.), vec3(1., 0., 1.),
//                             vec3(0., 1., 1.), vec3(1., 1., 1.),
//                             ],
//                             color_u8!(0, 200, 0, 255),
//                             mesh
//                         );
//                     }
//                     // SOUTH
//                     if !self.is_solid(x, y, z - 1) {
//                         self.add_face(
//                             pos,
//                             &[
//                             vec3(0., 0., 0.), vec3(1., 0., 0.),
//                             vec3(0., 1., 0.), vec3(1., 1., 0.),
//                             ],
//                             color_u8!(0, 170, 0, 255),
//                             mesh
//                         );
//                     }
//                     // EAST
//                     if !self.is_solid(x, y, z - 1) {
//                         self.add_face(
//                             pos,
//                             &[
//                             vec3(1., 0., 0.), vec3(1., 1., 0.),
//                             vec3(1., 0., 1.), vec3(1., 1., 1.),
//                             ],
//                             color_u8!(0, 250, 0, 255),
//                             mesh
//                         );
//                     }
//                     // EAST
//                     if !self.is_solid(x, y, z + 1) {
//                         self.add_face(
//                             pos,
//                             &[
//                             vec3(0., 0., 0.), vec3(0., 1., 0.),
//                             vec3(0., 0., 1.), vec3(0., 1., 1.),
//                             ],
//                             color_u8!(0, 250, 0, 255),
//                             mesh
//                         );
//                     }
//
//
//
//                 }
//             }
//         }
//     }
//
//     pub fn rebuild_mesh(&mut self) {
//         let vertices = Vec::new();
//         let indices = Vec::new();
//
//         let mut mesh = Mesh {
//             vertices,
//             indices,
//             texture: None,
//         };
//
//         self.build_faces(&mut mesh);
//         // self.build_faces(&mut mesh, Dir::Down);
//         // self.build_faces(&mut mesh, Dir::North);
//         // self.build_faces(&mut mesh, Dir::West);
//
//         self.mesh = Some(mesh);
//         // let mut vertecies = Vec::new();
//         // let mut indecies = Vec::<usize>::new();
//         //
//         // for x in 0..(CHUNK_SIZE-1) as u8 {
//         //     for y in 0..(WORLD_HEIGHT-1) as u8 {
//         //         for z in 0..(CHUNK_SIZE-1) as u8 {
//         //             // let vertecies = [
//         //             //     [x,   y,   z  ],
//         //             //     [x,   y,   z+1],
//         //             //     [x,   y+1, z  ],
//         //             //     [x,   y+1, z+1],
//         //             //     [x+1, y,   z  ],
//         //             //     [x+1, y,   z+1],
//         //             //     [x+1, y+1, z  ],
//         //             //     [x+1, y+1, z+1],
//         //             //
//         //             // ];
//         //             //
//         //             if self.is_solid(x, y, z) { continue; }
//         //
//         //
//         //             let cpos = [x, y, z];
//         //
//         //             {
//         //                 let mut pos = cpos.clone();
//         //                 for i in 0..2 {
//         //                     pos[i] = match pos[i].checked_sub(1) {
//         //                         Some(a) => a,
//         //                         None => pos[i]
//         //                     };
//         //                     if !self.is_solid(pos[0], pos[1], pos[2]) & !vertecies.contains(&pos) {
//         //                         pos[i] += 1;
//         //                         vertecies.push(pos);
//         //                         pos[i] += 1;
//         //
//         //                     } else {
//         //                         pos[i] += 2;
//         //                     }
//         //
//         //                     if !self.is_solid(pos[0], pos[1], pos[2]) & !vertecies.contains(&pos) {
//         //                         vertecies.push(pos);
//         //                     }
//         //                 }
//         //             }
//         //
//         //             // for pos in vertecies {
//         //             //     pos[0];
//         //             //     pos[1];
//         //             //     pos[2];
//         //             // }
//         //
//         //             if !self.is_solid(x, y, z) {
//         //
//         //             }
//         //         }
//         //     }
//         // }
//     }
//     //     let mut buffer = GreedyQuadsBuffer::new(PaddedChunkShape::SIZE as usize);
//     //
//     //     greedy_quads(
//     //         self.blocks.as_slice(),
//     //         &PaddedChunkShape {},
//     //         [0, 0, 0],
//     //         [15, 63, 15],
//     //         &RIGHT_HANDED_Y_UP_CONFIG.faces,
//     //         &mut buffer,
//     //     );
//     //
//     //     let mut vertices = Vec::new();
//     //     let mut indices = Vec::new();
//     //     let world_origin = vec3((chunk_pos.0 << 4 /*16*/) as f32, 0.0, (chunk_pos.1 << 4 /*16*/) as f32);
//     //
//     //     for (group_idx, face) in buffer.quads.groups.iter().enumerate() {
//     //         for quad in face {
//     //             let voxel = self.blocks[PaddedChunkShape::linearize(quad.minimum) as usize];
//     //             let color = if voxel.0 == 1 { GREEN } else { DARKBROWN };
//     //
//     //             let corners = RIGHT_HANDED_Y_UP_CONFIG.faces[group_idx].quad_corners(quad);
//     //             let base_idx = vertices.len() as u16;
//     //
//     //             for corner in corners {
//     //                 vertices.push(Vertex {
//     //                     position: world_origin + vec3((corner[0]) as f32, (corner[1]) as f32, (corner[2]) as f32),
//     //                     normal: vec4(0., 1., 0., 0.), 
//     //                     uv: vec2(0., 0.),
//     //                     color: color.into(),
//     //                 });
//     //             }
//     //
//     //             indices.extend_from_slice(&[
//     //                 base_idx, base_idx + 1, base_idx + 2,
//     //                 base_idx + 1, base_idx + 2, base_idx + 3,
//     //             ]);
//     //         }
//     //     }
//     //
//     //     if !vertices.is_empty() {
//     //         self.mesh = Some(Mesh { vertices, indices, texture: None });
//     //     }
//     //     self.dirty = false;
//     // }
// }
//
//
//
// /*
//    use macroquad::prelude::*;
//    use crate::systems::world_generation::{
//    CHUNK_SIZE,
//    WORLD_HEIGHT,
//    };
//
//    use block_mesh::{greedy_quads, GreedyQuadsBuffer, RIGHT_HANDED_Y_UP_CONFIG};

//    pub struct Chunk {
//    pub blocks: Box<[[[u8; CHUNK_SIZE as usize]; WORLD_HEIGHT as usize]; CHUNK_SIZE as usize]>,
//    pub mesh: Option<Mesh>,
//    pub dirty: bool,
//    }
//
//    impl Chunk {
//    pub fn new() -> Self {
//    Self {
//    blocks: Box::new([[[0; CHUNK_SIZE as usize]; WORLD_HEIGHT as usize]; CHUNK_SIZE as usize]),
//    mesh: None,
//    dirty: true,
//    }
//    }
//
//    pub fn rebuild_mesh(&mut self, chunk_pos: (i32, i32)) {
//    let mut buffer = GreedyQuadsBuffer::new(self.blocks.len());
//
//
//    }
//
//    pub fn _old_rebuild_mesh(&mut self, _chunk_pos: (i32, i32)) {
//    todo!("Original meshbuilder sucks.");
//    #[allow(unused)]
//    let mut vertices = Vec::new();
//    let mut indices = Vec::new();
//    let mut i = 0;
//
//    let world_origin = vec3(
//    (_chunk_pos.0 * CHUNK_SIZE) as f32,
//    0.0,
//    (_chunk_pos.1 * CHUNK_SIZE) as f32,
//    );
//
//    for x in 0..CHUNK_SIZE as usize {
//    for y in 0..WORLD_HEIGHT as usize {
//    for z in 0..CHUNK_SIZE as usize {
//    let block_id = self.blocks[x][y][z];
//    if block_id == 0 { continue; } // Skip Air
//
//    let p = world_origin + vec3(x as f32, y as f32, z as f32);
//    let color = if block_id == 1 { GREEN } else { DARKBROWN };
//
//    let directions = [
//    (vec3(0., 1., 0.),  y < (WORLD_HEIGHT-1) as usize && self.blocks[x][y+1][z] == 0), // Up
//    (vec3(0., -1., 0.), y > 0 && self.blocks[x][y-1][z] == 0),                         // Down
//    (vec3(1., 0., 0.),  x < (CHUNK_SIZE-1) as usize && self.blocks[x+1][y][z] == 0),   // Right
//    (vec3(-1., 0., 0.), x > 0 && self.blocks[x-1][y][z] == 0),                         // Left
//    (vec3(0., 0., 1.),  z < (CHUNK_SIZE-1) as usize && self.blocks[x][y][z+1] == 0),   // Forward
//    (vec3(0., 0., -1.), z > 0 && self.blocks[x][y][z-1] == 0),                         // Back
//    ];
//
//    for (dir, visible) in directions {
//    if visible {
//    self.add_face(&mut vertices, &mut indices, &mut i, p, dir, color);
//    }
//    println!("{}", i);
//    }
//    }
//    }
//    }
//
// self.mesh = Some(Mesh { vertices, indices, texture: None });
// self.dirty = false;
// }
//
// fn add_face(&self, verts: &mut Vec<Vertex>, idxs: &mut Vec<u16>, count: &mut u16, pos: Vec3, normal: Vec3, color: Color) {
//     for _ in 0..4 {
//         verts.push(Vertex {
//             position: pos,
//             normal: vec4(normal.x, normal.y, normal.z, 0.),
//             uv: vec2(0., 0.),
//             color: color.into(),
//         });
//     }
//     let i = *count;
//     idxs.extend_from_slice(&[i, i + 1, i + 2, i, i + 2, i + 3]);
//     *count += 4;
// }
// }
// */
