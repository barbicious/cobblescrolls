use crate::level::chunk::chunk_mesh::ChunkMesh;
use std::error::Error;
use std::rc::Rc;

pub mod chunk_mesh;

pub struct Chunk {
    chunk_mesh: ChunkMesh,
}

impl Chunk {
    pub const WIDTH: usize = 2;
    pub const HEIGHT: usize = 2;
    pub const DEPTH: usize = 2;

    pub fn new(gl: &Rc<glow::Context>, x: i32, y: i32, z: i32) -> Result<Self, Box<dyn Error>> {
        let chunk_mesh = ChunkMesh::new(gl, x, y, z)?;

        Ok(Self {
            chunk_mesh
        })
    }

    pub fn blit(&self) {
        self.chunk_mesh.blit();
    }
}