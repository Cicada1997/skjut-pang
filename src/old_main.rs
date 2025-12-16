use macroquad::prelude::*;
use hecs::*;

const MOVE_SPEED: f32 = 4.0;
const CAMERA_KEY_SPEED: f32 = 0.02;
const MOUSE_SENSATIVITY: f32 = 0.1;

struct Position(Vec3);
struct Speed(f32);
struct Rotation(Vec2);

fn conf() -> Conf {
    Conf {
        window_title: String::from("Now We Play"),
        window_width: 1260,
        window_height: 768,
        fullscreen: false,
        ..Default::default()
    }
}

pub struct Game {
    position: Vec3,
    yaw: f32,
    pitch: f32,

    front: Vec3,
    right: Vec3,
    up: Vec3,

    running: bool,
    grabbed: bool,
    world_up: Vec3,

    last_mouse_position: Vec2, 
}

impl Game {
    fn new(spawnpoint: Option<Vec3>) -> Self {
        let world_up: Vec3 = vec3(0.0, 1.0, 0.0);
        let yaw: f32 = 1.18; 
        let pitch: f32 = 0.0;

        let position = match spawnpoint {
            Some(pos) => pos,
            None => vec3(0.0, 1.0, 0.0),
        };

        let front = Game::calculate_front_vector(yaw, pitch);
        let right = front.cross(world_up).normalize();
        let up    = right.cross(front).normalize();

        let last_mouse_position: Vec2 = mouse_position().into();
        let grabbed = true;

        // let mut world = World::new();
        //
        // let player = world.spawn((
        //     position, 
        //     // Rotation, 
        //     // Speed
        // ));

        set_cursor_grab(grabbed);
        show_mouse(false);

        Self {
            position,
            yaw,
            pitch,
            front,
            right,
            up,
            running: true,
            grabbed,
            world_up,
            last_mouse_position,
        }
    }

    fn calculate_front_vector(yaw: f32, pitch: f32) -> Vec3 {
        vec3(
            yaw.cos() * pitch.cos(),
            pitch.sin(),
            yaw.sin() * pitch.cos(),
        )
        .normalize()
    }

    fn handle_input(&mut self) {
        if is_key_pressed(KeyCode::Escape) {
            self.running = false;
        }

        if is_key_pressed(KeyCode::Tab) {
            self.grabbed = !self.grabbed;
            set_cursor_grab(self.grabbed);
            show_mouse(!self.grabbed);
        }

        // så man inte går upp i luften :O)
        let flat_front = vec3(self.front.x, 0.0, self.front.z).normalize_or_zero();

        let delta = get_frame_time();
        let move_distance = MOVE_SPEED * delta;

        if is_key_down(KeyCode::Space) {
            self.position.y += move_distance;
        }

        if is_key_down(KeyCode::LeftControl) {
            self.position.y -= move_distance;
        }

        if is_key_down(KeyCode::W) {
            self.position += flat_front * move_distance;
        }

        if is_key_down(KeyCode::S) {
            self.position -= flat_front * move_distance;
        }

        if is_key_down(KeyCode::A) {
            self.position -= self.right * move_distance;
        }

        if is_key_down(KeyCode::D) {
            self.position += self.right * move_distance;
        }

        if is_key_down(KeyCode::Up) {
            self.pitch += CAMERA_KEY_SPEED; //* 0.035;
        }

        if is_key_down(KeyCode::Down) {
            self.pitch -= CAMERA_KEY_SPEED; //* 0.035;
        }

        if is_key_down(KeyCode::Left) {
            self.yaw -= CAMERA_KEY_SPEED; //* 0.035;
        }

        if is_key_down(KeyCode::Right) {
            self.yaw += CAMERA_KEY_SPEED; //* 0.035;
        }

    }

    // Huvudslingan
    async fn run(&mut self) {
        while self.running {
            let delta = get_frame_time();

            // --- 1. Hantera Inmatning och Musrörelse ---
            self.handle_input();

            let mouse_position: Vec2 = mouse_position().into();
            let mouse_delta = mouse_position - self.last_mouse_position;

            self.last_mouse_position = mouse_position;

            if self.grabbed {
                // Beräkna nya yaw och pitch
                self.yaw += mouse_delta.x * delta * MOUSE_SENSATIVITY;
                self.pitch -= mouse_delta.y * delta * MOUSE_SENSATIVITY;

                // Begränsa pitch (undvika kullerbyttor)
                self.pitch = self.pitch.clamp(-1.5, 1.5);

                self.front = Game::calculate_front_vector(self.yaw, self.pitch);
                self.right = self.front.cross(self.world_up).normalize();
                self.up = self.right.cross(self.front).normalize();
            }

            // --- 2. Rita Scenen ---
            clear_background(LIGHTGRAY);

            // Gå till 3D-läge
            set_camera(&Camera3D {
                position: self.position,
                up: self.up,
                target: self.position + self.front,
                ..Default::default()
            });

            // draw_grid(20, 1.0, BLACK, GRAY);

            let y = -0.5;
            for x in -10..10 {
                for z in -10..10 {
                    draw_cube(vec3(x as f32, y, z as f32), vec3(1., 0.01, 1.), None, GRAY);
                    draw_cube_wires(vec3(x as f32, y, z as f32), vec3(1., 0.01, 1.), WHITE);
                }
            }

            let reach_distance = vec3(5.0, 5.0, 5.0);
            draw_cube_wires(((self.front * reach_distance + self.position) + vec3(0.5, 0.5, 0.5)).floor(), vec3(1.0, 1.0, 1.0), RED);

            set_default_camera();

            draw_text("First Person Camera Example (W/A/S/D)", 10.0, 20.0, 30.0, BLACK);

            draw_text(
                format!("Pos X: {:.2} Y: {:.2} Z: {:.2}", self.position.x, self.position.y, self.position.z).as_str(),
                10.0,
                48.0 + 18.0,
                30.0,
                BLACK,
            );
            draw_text(
                format!("Press <TAB> to toggle mouse grab: {}", self.grabbed).as_str(),
                10.0,
                48.0 + 42.0,
                30.0,
                BLACK,
            );
            draw_text(
                format!("Yaw: {:.2} Pitch: {:.2}", self.yaw.to_degrees(), self.pitch.to_degrees()).as_str(),
                10.0,
                48.0 + 66.0,
                30.0,
                BLACK,
            );

            next_frame().await
        }
    }
}


#[macroquad::main(conf)]
async fn main() {
    let mut game = Game::new(None); 

    game.run().await;
}
