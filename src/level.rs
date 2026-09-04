use crate::level::chunk::Chunk;
use std::collections::HashMap;
use std::error::Error;
use std::rc::Rc;

pub mod chunk;
pub mod tile;

pub struct Level {
    chunks: HashMap<(i32, i32, i32), Chunk>,
}

impl Level {
    pub fn new(gl: &Rc<glow::Context>) -> Result<Self, Box<dyn Error>> {
        let mut chunks = HashMap::new();

        for z in -1..=1 {
            for y in -1..=1 {
                for x in -1..=1 {
                    let mut chunk = Chunk::new(gl, x, y, z)?;
                    chunk.regenerate_mesh();
                    chunks.insert((x, y, z), chunk);
                }
            }
        }

        Ok(Self { chunks })
    }

    pub fn blit(&self) {
        for (_, chunk) in self.chunks.iter() {
            chunk.blit()
        }
    }
}
