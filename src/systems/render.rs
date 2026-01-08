use macroquad::prelude::*;
use crate::components::physics::PhysicsObject;
use crate::components::graphics::GraphicsObject;
use crate::Game;
use crate::systems::world_generation::{CHUNK_SIZE, RENDER_DISTANCE};

impl Game {
    pub(crate) fn render(&mut self) {
        // Render Entities
        for (_eid, (graphics, physics)) in &mut self.ecs.query::<(&GraphicsObject, &PhysicsObject)>() {
            match graphics {
                GraphicsObject::Color(c) => {
                    draw_cube(physics.pos, vec3(1., 1., 1.), None, *c);
                }
                GraphicsObject::Image(_) => {
                    // Handle image rendering here
                }
            }
        }

        self.load_nearby_chunks();

        for ((_cx, _cz), chunk) in &mut self.world.chunks {
            if chunk.dirty {
                chunk.rebuild_mesh();
            }
            if let Some(mesh) = &chunk.mesh {
                draw_mesh(mesh);
            }
        }

        let len = self.fps_stats.len();
        self.fps_stats.push(get_fps());
        if len > 64 {
            self.fps_stats.swap(0, len);
        }

        let sum: i32 = self.fps_stats.iter().sum();
        let avg = if len != 0 {
            sum / len as i32
        } else { 0 };

        let _ = set_default_camera();

        let fps_text = format!("FPS: {:.2}", avg);
        let pos_str = match self.ecs.get::<&PhysicsObject>(self.player_id) {
            Ok(player) => {
                let x = format!("x: {:.2}", player.pos.x);
                let y = format!("y: {:.2}", player.pos.y);
                let z = format!("z: {:.2}", player.pos.z);

                format!("{} {} {}", x, y, z)
            },
            _ => {
                "pos undefined".to_string()
            },
        };

        draw_text(&fps_text, 10.0, 20.0, 20.0, WHITE);
        draw_text(&pos_str, 10.0, 40.0, 20.0, WHITE);
    }

    pub(crate) fn load_nearby_chunks(&mut self) {
        let p = self.ecs.get::<&PhysicsObject>(self.player_id).unwrap();
        
        let px = (p.pos.x / CHUNK_SIZE as f32).floor() as i32;
        let pz = (p.pos.z / CHUNK_SIZE as f32).floor() as i32;

        if px == self.last_render_cpos.x as i32 && pz == self.last_render_cpos.y as i32 {
            return;
        }
        self.last_render_cpos.x = px as f32;
        self.last_render_cpos.y = pz as f32;

        let mut chunk_num = 0;
        for x in -(RENDER_DISTANCE as i32)..RENDER_DISTANCE as i32 {
            for z in -(RENDER_DISTANCE as i32)..RENDER_DISTANCE as i32 {

                chunk_num += 1;
                self.world.load_or_gen(px + x, pz + z);
                print!("\rloaded chunks: {} / {}...", chunk_num, (2*RENDER_DISTANCE).pow(2))
            }
        }
    }
}
/*
use macroquad::prelude::*;

use crate::components::physics::{PhysicsObject};
use crate::components::graphics::{GraphicsObject};
use crate::color_u8;

use crate::Game;

use crate::systems::world_generation::{
    CHUNK_SIZE,
    // WORLD_HEIGHT,
    // ChunkPos,
    // chunk_pos_relative,
};

// pub static CHUNK_SIZE:      i32 = 24;
// pub static WORLD_HEIGHT:    i32 = 64;
pub static RENDER_DISTANCE:          i32 = 4;
pub static REACH_DISTANCE:           f32 = 6.;
pub static ENTITY_RENDER_DISTANCE:   f32 = 48.;
pub static FOG_FADE_DISTANCE:        f32 = 40.;

pub fn chunk_pos_relative(x: i32, y: i32) -> ((i32, i32), (i32, i32)) {
    let chunk_x = x / CHUNK_SIZE;
    let chunk_y = y / CHUNK_SIZE;
    let chunk_pos = (chunk_x, chunk_y);
    let local_pos = (x - CHUNK_SIZE*chunk_x, y - CHUNK_SIZE*chunk_y);

    return (chunk_pos, local_pos);
}

impl Game {
    pub(crate) fn render(&mut self) {
        for (_eid, (graphics, physics)) in &mut self.ecs.query::<(&GraphicsObject, &PhysicsObject)>() {
            if !self.in_render_distance(&physics.pos) {
                continue;
            }

            match graphics {
                GraphicsObject::Color(c) => {
                    let color = self.apply_fog(c, &physics.pos);
                    draw_cube(physics.pos, physics.size, None, color);
                }

                GraphicsObject::Image(i) => {
                    draw_cube(physics.pos, physics.size, Some(i), color_u8(0., 0., 0., 0.));
                }

                _ => panic!("fuck: Unknown rendering type")
            }
        }

        self.load_nearby_chunks();

        for ((cx, cz), chunk) in &mut self.world.chunks {
            if chunk.dirty {
                chunk.rebuild_mesh((*cx, *cz));
            }
        }

        // Use the world's optimized render call
        self.world.render();
        self.render_selected();
    }

    pub fn apply_fog(&self, c: &Color, pos: &Vec3) -> Color {
        let player_phys = self.ecs.get::<&PhysicsObject>(self.player_id).unwrap();
        let m           = player_phys.pos.distance(*pos) / ENTITY_RENDER_DISTANCE;
        let a           = 1. - (player_phys.pos.distance(*pos) - FOG_FADE_DISTANCE).clamp(0., ENTITY_RENDER_DISTANCE) / (ENTITY_RENDER_DISTANCE - FOG_FADE_DISTANCE);

        Color { 
            r: (c.r - m),//.clamp(0., 1.),
            g: (c.g - m),//.clamp(0., 1.),
            b: (c.b - m),//.clamp(0., 1.),
            a: (a).clamp(0., 1.),
        }
    }

    pub fn in_render_distance(&self, pos: &Vec3) -> bool {
        let player_phys = self.ecs.get::<&PhysicsObject>(self.player_id).unwrap();
        player_phys.pos.distance(*pos) < ENTITY_RENDER_DISTANCE
    }

    // pub(crate) fn draw_chunk(&mut self, pos: &ChunkPos) {
    //     dbg!(pos);
    //     for x in 0..CHUNK_SIZE {
    //         for y in 0..WORLD_HEIGHT {
    //             for z in 0..CHUNK_SIZE {
    //                 warn!("Implement error handling");
    //                 if let Some(chunk) = self.world.chunks.get(&chunk_pos_relative(x, y).0) {
    //                     let block = self.world.fetch_block(chunk, x, y, z).unwrap();
    //
    //                     dbg!(block);
    //
    //                     let position = vec3(
    //                         (x + pos.0 * CHUNK_SIZE) as f32, 
    //                         y as f32, 
    //                         (z + pos.0 * CHUNK_SIZE) as f32
    //                     );
    //
    //                     dbg!(position);
    //
    //                     draw_cube(position, vec3(1., 1., 1.), None, block.color);
    //                 }
    //             }
    //         }
    //     }
    // }

    // pub(crate) fn load_nearby_chunks(&mut self) {
    //     let (pcx, pcz) = {
    //         let p = self.ecs.get::<&PhysicsObject>(self.player_id).unwrap();
    //         let (mut p_chunk_pos, _p_chunk_local) = chunk_pos_relative(p.pos.x as i32, p.pos.z as i32);
    //
    //         p_chunk_pos
    //     };
    //
    //     //     p_chunk_pos.0 += RENDER_DISTANCE;
    //     //     p_chunk_pos.1 += RENDER_DISTANCE;
    //     //
    //     //     p_chunk_pos
    //     // };
    //
    //     // dbg!(cx);
    //     // dbg!(cz);
    //
    //
    //     for x in -RENDER_DISTANCE..RENDER_DISTANCE {
    //         for z in -RENDER_DISTANCE..RENDER_DISTANCE {
    //             let p = self.ecs.get::<&PhysicsObject>(self.player_id).unwrap();
    //             if vec2((x + CHUNK_SIZE / 2) as f32, (z + CHUNK_SIZE / 2) as f32).distance(vec2(p.pos.x, p.pos.z)) > ENTITY_RENDER_DISTANCE { continue; }
    //
    //             let cx = x + pcx;
    //             let cz = z + pcz;
    //
    //             self.world.load_or_gen(cx, cz);
    //             // println!("Successfully loaded chunk {x}, {x}.");
    //         }
    //     }
    // }
    pub(crate) fn load_nearby_chunks(&mut self) {
        let p = self.ecs.get::<&PhysicsObject>(self.player_id).unwrap();

        // Correctly calculate player's chunk index using floor division
        let px = (p.pos.x / CHUNK_SIZE as f32).floor() as i32;
        let pz = (p.pos.z / CHUNK_SIZE as f32).floor() as i32;

        for x in -RENDER_DISTANCE..RENDER_DISTANCE {
            for z in -RENDER_DISTANCE..RENDER_DISTANCE {
                // Pass index, not world coordinates
                self.world.load_or_gen(px + x, pz + z);
            }
        }
    }

    // pub(crate) fn render_world() {
    //     let y = -0.5;
    //     for x in -10..10 {
    //         for z in -10..10 {
    //             draw_cube(vec3(x as f32, y, z as f32), vec3(1., 0.01, 1.), None, GRAY);
    //             draw_cube_wires(vec3(x as f32, y, z as f32), vec3(1., 0.01, 1.), WHITE);
    //         }
    //     }
    // }

    pub(crate) fn render_selected(&mut self) {
        let physics = self.ecs.get::<&PhysicsObject>(self.player_id).unwrap();

        draw_cube_wires(((physics.front() * REACH_DISTANCE + physics.pos) + vec3(0.5, 0.5, 0.5)).floor(), vec3(1.0, 1.0, 1.0), RED);
    }
}
*/
