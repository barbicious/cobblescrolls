use crate::level::chunk::chunk_mesh::ChunkMesh;
use std::error::Error;
use std::rc::Rc;
use crate::level::tile::TileType;

pub mod chunk_mesh;

pub struct Chunk {
    tiles: [TileType; Self::WIDTH * Self::HEIGHT * Self::DEPTH],
    chunk_mesh: ChunkMesh,
    x: i32,
    y: i32,
    z: i32,
}

impl Chunk {
    pub const WIDTH: usize = 8;
    pub const HEIGHT: usize = 8;
    pub const DEPTH: usize = 8;

    pub fn new(gl: &Rc<glow::Context>, x: i32, y: i32, z: i32) -> Result<Self, Box<dyn Error>> {
        let mut tiles = [TileType::Air; Self::WIDTH * Self::HEIGHT * Self::DEPTH];

        for z in 0..Self::DEPTH {
            for y in 0..Self::HEIGHT {
                for x in 0..Self::WIDTH {
                    tiles[Self::idx(x, y, z)] = if y > 6 {
                        TileType::Grass
                    } else {
                        TileType::Stone
                    }
                }
            }
        }

        let mut chunk_mesh = ChunkMesh::new(gl, tiles, x, y, z)?;
        
        chunk_mesh.regenerate_mesh(&tiles);
        
        Ok(Self {
            chunk_mesh,
            tiles,
            x,
            y,
            z,
        })
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