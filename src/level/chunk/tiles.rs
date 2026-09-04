use std::ops::{Index, IndexMut};
use crate::level::chunk::Chunk;
use crate::level::tile::TileType;

pub struct Tiles {
    tiles: [TileType; Chunk::WIDTH * Chunk::HEIGHT * Chunk::DEPTH],
}

impl Tiles {
    pub fn new() -> Self {
        let mut tiles = [TileType::Air; Chunk::WIDTH * Chunk::HEIGHT * Chunk::DEPTH];

        for z in 0..Chunk::DEPTH {
            for y in 0..Chunk::HEIGHT {
                for x in 0..Chunk::WIDTH {
                    tiles[Chunk::idx(x, y, z)] = if y > 6 {
                        TileType::Grass
                    } else {
                        TileType::Stone
                    }
                }
            }
        }

        Self {
            tiles,
        }
    }

    pub fn is_tile_transparent(&self, x: i32, y: i32, z: i32) -> bool {
        if x < 0 || x > (Chunk::WIDTH - 1) as i32 {
            return true
        }

        if y < 0 || y > (Chunk::HEIGHT - 1) as i32 {
            return true
        }

        if z < 0 || z > (Chunk::DEPTH - 1) as i32 {
            return true
        }

        matches!(self[Chunk::idx(x as usize, y as usize, z as usize)], TileType::Air)
    }
}

impl Index<usize> for Tiles {
    type Output = TileType;

    fn index(&self, index: usize) -> &Self::Output {
        &self.tiles[index]
    }
}

impl IndexMut<usize> for Tiles {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.tiles[index]
    }
}