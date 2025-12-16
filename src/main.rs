use macroquad::prelude::*;
use hecs;
use hecs::Entity as EntityId;

mod systems;

mod components;
use components::physics::PhysicsObject;
use components::graphics::GraphicsObject;

pub fn color_u8(r: f32, g: f32, b: f32, a: f32) -> Color {
    Color { r: r / 255., g: g / 255., b: b / 255., a }
}

pub struct Game {
    ecs: hecs::World,
    camera: Camera3D,

    player_id: EntityId,

    // State
    running: bool,
    grabbed: bool,
}

impl Game {
    pub fn new() -> Self {
        let mut ecs        = hecs::World::new();

        let world_up       = vec3(0.0, 1.0, 0.0);

        let player         = PhysicsObject::new(world_up, None);
        let camera         = Camera3D::default();


        let player_id      = ecs.spawn((player,));

        let running        = false;
        let grabbed        = false;

        Self { ecs, camera, player_id, running, grabbed }
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
