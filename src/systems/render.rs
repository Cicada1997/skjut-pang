use macroquad::prelude::*;

use crate::components::physics::{PhysicsObject};
use crate::components::graphics::{GraphicsObject};
use crate::color_u8;

use crate::Game;

static REACH_DISTANCE: f32 = 6.;

impl Game {
    pub(crate) fn render(&mut self) {
        for (_eid, (graphics, physics)) in &mut self.ecs.query::<(&GraphicsObject, &PhysicsObject)>() {
            match graphics {
                GraphicsObject::Color(c) => {
                    draw_cube(physics.pos, physics.size, None, *c);
                }

                GraphicsObject::Image(i) => {
                    draw_cube(physics.pos, physics.size, Some(i), color_u8(0., 0., 0., 0.));
                }

                _ => panic!("fuck")
           }
        }

        Self::render_world();
        self.render_selected();
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
