use std::env::args;

use macroquad::prelude::*;
use hecs;
use hecs::Entity as EntityId;

mod systems;
use systems::world_generation;

mod components;
use components::physics::PhysicsObject;
use components::graphics::GraphicsObject;

pub fn color_u8(r: f32, g: f32, b: f32, a: f32) -> Color {
    Color { r: r / 255., g: g / 255., b: b / 255., a }
}

pub struct Game {
    ecs: hecs::World,
    world: world_generation::World,
    camera: Camera3D,

    player_id: EntityId,

    fps_stats: Vec<i32>,

    // State
    running: bool,
    grabbed: bool,

    last_render_cpos: Vec2,
}

pub fn radian(deg: f32) -> f32 {
    deg * std::f32::consts::PI / 180.0
}

impl Game {
    pub fn new() -> Self {
        let mut ecs        = hecs::World::new();

        let seed           = match args().nth(1) {
            Some(seed_str) => seed_str.parse::<u32>().unwrap(),
            None           => 2,
        };

        let world          = world_generation::World::new(seed);

        let world_up       = vec3(0.0, 1.0, 0.0);

        let player         = PhysicsObject::new(world_up, None);
        let camera         = Camera3D {
            fovy: radian(110.),
            ..Default::default()
        };


        let player_id      = ecs.spawn((player,));

        let fps_stats      = Vec::new();

        let running        = false;
        let grabbed        = true;

        let last_render_cpos = Vec2::new(0., 0.);

        Self { ecs, world, camera, player_id, fps_stats, running, grabbed, last_render_cpos }
    }

    fn setup(&mut self) {
        self.running = true;

        set_cursor_grab(self.grabbed);
        show_mouse(false);

        self.ecs.spawn_batch(
            (0..15).map(|i| ( 
                PhysicsObject::new(
                    vec3(0., 1., 0.), 
                    Some(vec3(i as f32, 0., i as f32))
                ),
                GraphicsObject::Color(color_u8(0., 255., 0., 1.))
                )
            )
        );
    }

    fn pipeline(&mut self) {
        self.player_updates();
        self.player_movement();

        self.render();
    }

    pub async fn run(&mut self) {
        self.setup();

        while self.running {
            clear_background(color_u8(70., 70., 255., 0.));

            if is_key_down(KeyCode::Escape) { break }

            self.pipeline();

            next_frame().await;
        }
    }
}

fn conf() -> Conf {
    Conf {
        window_title: String::from("стрелять бах"),
        window_width: 1260,
        window_height: 768,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let mut game = Game::new(); 

    game.run().await;
}
