use crate::level::NeighboringTiles;
use crate::level::chunk::Chunk;
use crate::level::tile::TileType;
use std::ops::{Index, IndexMut};

macro_rules! test_bounds {
    ($eval:expr, $tiles:expr, $x:expr, $y:expr, $z:expr) => {
        if $eval {
            return if let Some(tiles) = $tiles {
                matches!(
                    tiles[Chunk::idx($x as usize, $y as usize, $z as usize)],
                    TileType::Air
                )
            } else {
                true
            };
        }
    };
}

pub struct Tiles {
    tiles: [TileType; Chunk::WIDTH * Chunk::HEIGHT * Chunk::DEPTH],
}

impl Tiles {
    pub fn new(cx: i32, cy: i32, cz: i32) -> Self {
        let mut tiles = [TileType::Air; Chunk::WIDTH * Chunk::HEIGHT * Chunk::DEPTH];

        for z in 0..Chunk::DEPTH {
            for y in 0..Chunk::HEIGHT {
                for x in 0..Chunk::WIDTH {
                    tiles[Chunk::idx(x, y, z)] = if (y as i32 + cy * Chunk::HEIGHT as i32) < 6 {
                        TileType::Stone
                    } else if (y as i32 + cy * Chunk::HEIGHT as i32) < 7 {
                        TileType::Grass
                    } else {
                        TileType::Air
                    }
                }
            }
        }

        Self { tiles }
    }

    pub fn is_tile_transparent(
        &self,
        x: i32,
        y: i32,
        z: i32,
        neighboring_tiles: &NeighboringTiles,
    ) -> bool {
        test_bounds!(x < 0, neighboring_tiles.left_tiles, Chunk::WIDTH - 1, y, z);
        test_bounds!(
            x > (Chunk::WIDTH - 1) as i32,
            neighboring_tiles.right_tiles,
            0,
            y,
            z
        );

        test_bounds!(y < 0, neighboring_tiles.up_tiles, x, Chunk::HEIGHT - 1, z);
        test_bounds!(
            y > (Chunk::HEIGHT - 1) as i32,
            neighboring_tiles.bottom_tiles,
            x,
            0,
            z
        );

        test_bounds!(z < 0, neighboring_tiles.back_tiles, x, y, Chunk::DEPTH - 1);
        test_bounds!(
            z > (Chunk::DEPTH - 1) as i32,
            neighboring_tiles.front_tiles,
            x,
            y,
            0
        );

        matches!(
            self[Chunk::idx(x as usize, y as usize, z as usize)],
            TileType::Air
        )
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
