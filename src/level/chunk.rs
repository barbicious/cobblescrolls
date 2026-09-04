use crate::level::chunk::chunk_mesh::ChunkMesh;
use crate::level::tile::TileType;
use std::error::Error;
use std::rc::Rc;
use crate::level::chunk::tiles::Tiles;

pub mod chunk_mesh;
pub mod tiles;

pub struct Chunk {
    chunk_mesh: ChunkMesh,
    tiles: Tiles,
    x: i32,
    y: i32,
    z: i32,
}

impl Chunk {
    pub const WIDTH: usize = 8;
    pub const HEIGHT: usize = 8;
    pub const DEPTH: usize = 8;

    pub fn new(gl: &Rc<glow::Context>, x: i32, y: i32, z: i32) -> Result<Self, Box<dyn Error>> {
        let tiles = Tiles::new();

        let chunk_mesh = ChunkMesh::new(gl, x, y, z)?;

        Ok(Self {
            chunk_mesh,
            tiles,
            x,
            y,
            z,
        })
    }

    pub fn regenerate_mesh(&mut self) {
        self.chunk_mesh.regenerate_mesh(&self.tiles);
    }

    pub fn tile_at(&self, x: usize, y: usize, z: usize) -> TileType {
        self.tiles[Self::idx(x, y, z)]
    }

    pub fn set_tile(&mut self, x: usize, y: usize, z: usize, tile_type: TileType) {
        self.tiles[Self::idx(x, y, z)] = tile_type;

        self.chunk_mesh.regenerate_mesh(&self.tiles)
    }

    const fn idx(x: usize, y: usize, z: usize) -> usize {
        x + Self::WIDTH * (y + Self::HEIGHT * z)
    }

    pub fn blit(&self) {
        self.chunk_mesh.blit();
    }
}
