use crate::world::player::camera::Camera;
use crate::datatype::Rotation;

use na::{
    Point3,
};

use std::fmt::{Debug, Formatter};
use std::fmt;


pub mod camera;


// chunk radius in chunk size
pub const CHUNK_RADIUS: u32 = 2;
// the radius of which the world.player can edit the world
pub const EDIT_RADIUS: u32 = 10;


#[derive(Clone, PartialEq)]
pub struct Player {
    pub camera: Camera,
}

impl Player {
    pub fn new() -> Self {
        Self {
            camera: Camera::new(
                0.004,
                0.1,
                // to avoid the negative perlin noise for now
                Point3::new(1000.0, 64.0, 1000.0),
                Rotation::new(0.0, 0.0, 0.0),
            )
        }
    }
}

impl Debug for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Player")
            .field("position", &self.camera.position)
            .field("rotation", &self.camera.rotation)
            .finish()
    }
}
