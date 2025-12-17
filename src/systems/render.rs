use macroquad::prelude::*;

use crate::components::physics::{PhysicsObject};
use crate::components::graphics::{GraphicsObject};
use crate::color_u8;

use crate::Game;

use crate::systems::world_generation::{
    CHUNK_SIZE,
    WORLD_HEIGHT,
    ChunkPos,
    chunk_pos_relative,
};

pub static REACH_DISTANCE:    f32 = 6.;
pub static ENTITY_RENDER_DISTANCE:   f32 = 48.;
pub static FOG_FADE_DISTANCE: f32 = 40.;

impl Game {
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


        Self::render_world();
        println!("rendering world...");
        for chunk in self.load_nearby_chunks().iter() {
            dbg!(chunk);
            self.draw_chunk(&chunk);
        }
        println!("rendered world");

        self.render_selected();
    }

    pub(crate) fn draw_chunk(&mut self, pos: &ChunkPos) {
        dbg!(pos);
        for x in 0..CHUNK_SIZE {
            for y in 0..WORLD_HEIGHT {
                for z in 0..CHUNK_SIZE {
                    warn!("Implement error handling");
                    if let Some(chunk) = self.world.chunks.get(&chunk_pos_relative(x, y).0) {
                        let block = self.world.fetch_block(chunk, x, y, z).unwrap();

                        dbg!(block);

                        let position = vec3(
                            (x + pos.0 * CHUNK_SIZE) as f32, 
                            y as f32, 
                            (z + pos.0 * CHUNK_SIZE) as f32
                        );

                        dbg!(position);

                        draw_cube(position, vec3(1., 1., 1.), None, block.color);
                    }
                }
            }
        }
    }

    pub(crate) fn render_world() {
        let y = -0.5;
        for x in -10..10 {
            for z in -10..10 {
                draw_cube(vec3(x as f32, y, z as f32), vec3(1., 0.01, 1.), None, GRAY);
                draw_cube_wires(vec3(x as f32, y, z as f32), vec3(1., 0.01, 1.), WHITE);
            }
        }
    }

    pub(crate) fn render_selected(&mut self) {
        let physics = self.ecs.get::<&PhysicsObject>(self.player_id).unwrap();

        draw_cube_wires(((physics.front() * REACH_DISTANCE + physics.pos) + vec3(0.5, 0.5, 0.5)).floor(), vec3(1.0, 1.0, 1.0), RED);
    }
}
