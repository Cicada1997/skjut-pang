#![allow(unused)]
use macroquad::prelude::*;

#[derive(Debug, Clone)]
pub struct PhysicsObject {
    pub pos: Vec3,
    pub size: Vec3,
    
    pub yaw: f32,
    pub pitch: f32,

    pub world_up: Vec3,

    pub on_ground: bool,
}

impl PhysicsObject {
    pub fn new(world_up: Vec3, spawnpoint: Option<Vec3>) -> Self {
        
        let yaw: f32 = 1.18; 
        let pitch: f32 = 0.0;

        let pos = match spawnpoint {
            Some(pos) => pos,
            None => vec3(0.0, 1.0, 0.0),
        };

        let size = vec3(1.0,1.0, 1.0);
        let on_ground = false;

        Self { pos, size, yaw, pitch, world_up, on_ground }
    }

    pub(crate) fn center_camera(&self, camera: &mut Camera3D) {
        camera.position = self.pos;
        camera.up       = self.up();
        camera.target   = self.pos + self.front();
    }

    pub(crate) fn front(&self) -> Vec3 {
        vec3(
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.sin() * self.pitch.cos(),
        )
        .normalize()
    }

    pub(crate) fn right(&self) -> Vec3 {
        self.front().cross(self.world_up).normalize()
    }
    pub(crate) fn up(&self) -> Vec3 {
        self.right().cross(self.front()).normalize()
    }
    
}
