use crate::level::NeighboringTiles;
use crate::level::chunk::chunk_mesh::ChunkMesh;
use crate::level::chunk::tiles::Tiles;
use crate::level::tile::TileType;
use std::error::Error;
use std::rc::Rc;

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
        let tiles = Tiles::new(x, y, z);

        let chunk_mesh = ChunkMesh::new(gl, x, y, z)?;

        Ok(Self {
            chunk_mesh,
            tiles,
            x,
            y,
            z,
        })
    }

    pub fn tiles(&self) -> &Tiles {
        &self.tiles
    }

    pub fn relative_position(&self, ox: i32, oy: i32, oz: i32) -> (i32, i32, i32) {
        (self.x + ox, self.y + oy, self.z + oz)
    }

    pub fn regenerate_mesh(&mut self, neighboring_tiles: &NeighboringTiles) {
        self.chunk_mesh
            .regenerate_mesh(&self.tiles, neighboring_tiles);
    }

    pub fn tile_at(&self, x: usize, y: usize, z: usize) -> TileType {
        self.tiles[Self::idx(x, y, z)]
    }

    pub fn set_tile(
        &mut self,
        x: usize,
        y: usize,
        z: usize,
        tile_type: TileType,
        neighboring_tiles: &NeighboringTiles,
    ) {
        self.tiles[Self::idx(x, y, z)] = tile_type;

        self.chunk_mesh
            .regenerate_mesh(&self.tiles, neighboring_tiles)
    }

    const fn idx(x: usize, y: usize, z: usize) -> usize {
        x + Self::WIDTH * (y + Self::HEIGHT * z)
    }

    pub fn blit(&self) {
        self.chunk_mesh.blit();
    }
}
