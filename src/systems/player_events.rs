use macroquad::prelude::*;
use macroquad::input::{ is_key_down, mouse_delta_position, KeyCode };

use crate::Game;
use crate::{ set_camera, get_frame_time };

use crate::components::physics::PhysicsObject;

static MOUSE_SENSATIVITY:  f32 = 100.2;
static CAMERA_SENSATIVITY: f32 = 0.04;
static MOVEMENT_SPEED:     f32 = 6.2;

impl Game {
    pub(crate) fn player_updates(&mut self) {
        match self.ecs.query_one_mut::<&mut PhysicsObject>(self.player_id) {
            Ok(player) => {
                player.center_camera(&mut self.camera);
                set_camera(&self.camera);
            },
            Err(_) => {},
        }
    }

    pub(crate) fn player_movement(&mut self) {
        match self.ecs.query_one_mut::<&mut PhysicsObject>(self.player_id) {
            Ok(player) => {
                let delta = get_frame_time();
                let mouse_delta = mouse_delta_position();

                let front = player.front();
                let flat_front = vec3(front.x, 0.0, front.z);


                // Player Movement
                if is_key_down(KeyCode::W) {
                    player.pos += flat_front * delta * MOVEMENT_SPEED;
                }

                if is_key_down(KeyCode::S) {
                    player.pos -= flat_front * delta * MOVEMENT_SPEED;
                }

                if is_key_down(KeyCode::A) {
                    player.pos -= player.right() * delta * MOVEMENT_SPEED;
                }

                if is_key_down(KeyCode::D) {
                    player.pos += player.right() * delta * MOVEMENT_SPEED;
                }

                
                // Vertical Movement
                if is_key_down(KeyCode::Space) {
                    player.pos.y += delta * MOVEMENT_SPEED;
                }

                if is_key_down(KeyCode::LeftControl) {
                    player.pos.y -= delta * MOVEMENT_SPEED;
                }


                // Camera Movement
                if is_key_down(KeyCode::Up) {
                    player.pitch += CAMERA_SENSATIVITY;
                }

                if is_key_down(KeyCode::Down) {
                    player.pitch -= CAMERA_SENSATIVITY;
                }

                if is_key_down(KeyCode::Left) {
                    player.yaw -= CAMERA_SENSATIVITY;
                }

                if is_key_down(KeyCode::Right) {
                    player.yaw += CAMERA_SENSATIVITY;
                }

                player.yaw   -= mouse_delta.x * delta * MOUSE_SENSATIVITY;
                player.pitch += mouse_delta.y * delta * MOUSE_SENSATIVITY;

                player.pitch  = player.pitch.clamp(-1.5, 1.5);
            },
            Err(_) => {},
        }
    }
}
