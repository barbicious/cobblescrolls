use crate::level::chunk::Chunk;
use crate::level::chunk::tiles::Tiles;
use crate::level::tile::TileType;
use std::collections::HashMap;
use std::error::Error;
use std::rc::Rc;

pub mod chunk;
pub mod tile;

macro_rules! relative_tiles {
    ($pos:expr, $chunks:expr) => {
        if let Some(chunk) = $chunks.get($pos) {
            Some(chunk.tiles())
        } else {
            None
        }
    };
}

macro_rules! world_to_chunk_pos {
    ($chunk_x:ident, $chunk_y:ident, $chunk_z:ident, $x:expr, $y:expr, $z:expr) => {
        let $chunk_x = chunk_handle_negative_coordinates($x);
        let $chunk_y = chunk_handle_negative_coordinates($y);
        let $chunk_z = chunk_handle_negative_coordinates($z);
    };
}

macro_rules! world_to_tile_position {
    ($tile_x:ident, $tile_y:ident, $tile_z:ident, $x:expr, $y:expr, $z:expr) => {
        let $tile_x = tile_handle_negative_coordinates($x);
        let $tile_y = tile_handle_negative_coordinates($y);
        let $tile_z = tile_handle_negative_coordinates($z);
    };
}

const fn tile_handle_negative_coordinates(n: i32) -> usize {
    if n < 0 {
        (n + (Chunk::WIDTH as i32 * chunk_handle_negative_coordinates(n)).abs()) as usize
    } else {
        n as usize % Chunk::WIDTH
    }
}

const fn chunk_handle_negative_coordinates(n: i32) -> i32 {
    if n < 0 {
        ((n - 1) / Chunk::WIDTH as i32) - 1
    } else {
        n / Chunk::WIDTH as i32
    }
}

pub struct NeighboringTiles<'a> {
    pub up_tiles: Option<&'a Tiles>,
    pub bottom_tiles: Option<&'a Tiles>,
    pub left_tiles: Option<&'a Tiles>,
    pub right_tiles: Option<&'a Tiles>,
    pub front_tiles: Option<&'a Tiles>,
    pub back_tiles: Option<&'a Tiles>,
}

impl<'a> NeighboringTiles<'a> {
    pub fn new(chunk: &Chunk, chunks: &'a HashMap<(i32, i32, i32), Chunk>) -> Self {
        let right_tiles = relative_tiles!(&chunk.relative_position(1, 0, 0), &chunks);
        let left_tiles = relative_tiles!(&chunk.relative_position(-1, 0, 0), &chunks);
        let up_tiles = relative_tiles!(&chunk.relative_position(0, -1, 0), &chunks);
        let bottom_tiles = relative_tiles!(&chunk.relative_position(0, 1, 0), &chunks);
        let back_tiles = relative_tiles!(&chunk.relative_position(0, 0, -1), &chunks);
        let front_tiles = relative_tiles!(&chunk.relative_position(0, 0, 1), &chunks);

        Self {
            up_tiles,
            back_tiles,
            bottom_tiles,
            left_tiles,
            right_tiles,
            front_tiles,
        }
    }
}

pub struct Level {
    chunks: HashMap<(i32, i32, i32), Chunk>,
}

impl Level {
    pub fn new(gl: &Rc<glow::Context>) -> Result<Self, Box<dyn Error>> {
        let mut chunks = HashMap::new();

        let mut meshes = Vec::new();

        for z in -1..=1 {
            for y in -1..=1 {
                for x in -1..=1 {
                    let chunk = Chunk::new(gl, x, y, z)?;

                    meshes.push((x, y, z));

                    chunks.insert((x, y, z), chunk);
                }
            }
        }

        for mesh in meshes {
            if let Some(mut chunk) = chunks.remove(&mesh) {
                chunk.regenerate_mesh(&NeighboringTiles::new(&chunk, &chunks));

                chunks.insert(mesh, chunk);
            }
        }

        Ok(Self { chunks })
    }

    pub fn chunk_at(&self, x: i32, y: i32, z: i32) -> &Chunk {
        self.chunks.get(&(x, y, z)).unwrap()
    }

    pub fn get_tile(&mut self, x: i32, y: i32, z: i32) -> TileType {
        world_to_chunk_pos!(chunk_x, chunk_y, chunk_z, x, y, z);

        world_to_tile_position!(tile_x, tile_y, tile_z, x, y, z);

        println!("{tile_x} {tile_y} {tile_z}");

        self.chunks
            .get(&(chunk_x, chunk_y, chunk_z))
            .unwrap()
            .tile_at(                tile_x,
                                     tile_y,
                                     tile_z,
            )
    }

    pub fn set_tile(&mut self, x: i32, y: i32, z: i32, tile_type: TileType) {
        world_to_chunk_pos!(chunk_x, chunk_y, chunk_z, x, y, z);

        world_to_tile_position!(tile_x, tile_y, tile_z, x, y, z);

        if let Some(mut chunk) = self.chunks.remove(&(chunk_x, chunk_y, chunk_z)) {
            chunk.set_tile(
                tile_x,
                tile_y,
                tile_z,
                tile_type,
                &NeighboringTiles::new(&chunk, &self.chunks),
            );

            self.chunks.insert((chunk_x, chunk_y, chunk_z), chunk);
        }
    }

    pub fn blit(&self) {
        for (_, chunk) in self.chunks.iter() {
            chunk.blit()
        }
    }
}
