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

#[macro_export]
macro_rules! world_to_chunk_pos {
    ($chunk_x:ident, $chunk_y:ident, $chunk_z:ident, $x:expr, $y:expr, $z:expr) => {
        let $chunk_x = crate::level::chunk_handle_negative_coordinates($x);
        let $chunk_y = crate::level::chunk_handle_negative_coordinates($y);
        let $chunk_z = crate::level::chunk_handle_negative_coordinates($z);
    };
}

macro_rules! world_to_tile_pos {
    ($tile_x:ident, $tile_y:ident, $tile_z:ident, $x:expr, $y:expr, $z:expr) => {
        let $tile_x = crate::level::tile_handle_negative_coordinates($x);
        let $tile_y = crate::level::tile_handle_negative_coordinates($y);
        let $tile_z = crate::level::tile_handle_negative_coordinates($z);
    };
}

const fn tile_handle_negative_coordinates(n: i32) -> usize {
    n.rem_euclid(Chunk::WIDTH as i32) as usize
}

pub const fn chunk_handle_negative_coordinates(n: i32) -> i32 {
    n.div_euclid(Chunk::WIDTH as i32)
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

type ChunkPos = (i32, i32, i32);

pub struct Level {
    chunks: HashMap<ChunkPos, Chunk>,
    mesh_queue: Vec<ChunkPos>,
    gl: Rc<glow::Context>,
}

impl Level {
    const RENDER_DISTANCE: i32 = 4;
    
    pub fn new(gl: &Rc<glow::Context>) -> Result<Self, Box<dyn Error>> {
        let gl = gl.clone();

        let mut chunks = HashMap::new();

        let mut mesh_queue = Vec::new();

        for z in -Self::RENDER_DISTANCE..=Self::RENDER_DISTANCE {
            for y in -Self::RENDER_DISTANCE..=Self::RENDER_DISTANCE {
                for x in -Self::RENDER_DISTANCE..=Self::RENDER_DISTANCE {
                    let chunk = Chunk::new(&gl, x, y, z)?;

                    mesh_queue.push((x, y, z));

                    chunks.insert((x, y, z), chunk);
                }
            }
        }

        Ok(Self { chunks, mesh_queue, gl })
    }

    pub fn chunk_at(&self, x: i32, y: i32, z: i32) -> &Chunk {
        self.chunks.get(&(x, y, z)).unwrap()
    }

    pub fn get_tile(&mut self, x: i32, y: i32, z: i32) -> TileType {
        world_to_chunk_pos!(chunk_x, chunk_y, chunk_z, x, y, z);

        world_to_tile_pos!(tile_x, tile_y, tile_z, x, y, z);

        self.chunks
            .get(&(chunk_x, chunk_y, chunk_z))
            .unwrap()
            .tile_at(tile_x, tile_y, tile_z)
    }

    pub fn set_tile(&mut self, x: i32, y: i32, z: i32, tile_type: TileType) {
        world_to_chunk_pos!(chunk_x, chunk_y, chunk_z, x, y, z);

        world_to_tile_pos!(tile_x, tile_y, tile_z, x, y, z);

        if let Some(mut chunk) = self.chunks.remove(&(chunk_x, chunk_y, chunk_z)) {
            chunk.set_tile(
                tile_x,
                tile_y,
                tile_z,
                tile_type,
                &NeighboringTiles::new(&chunk, &self.chunks),
            );

            if !self.mesh_queue.contains(&(chunk_x, chunk_y, chunk_z)) {
                self.mesh_queue.push((chunk_x, chunk_y, chunk_z))
            }

            self.chunks.insert((chunk_x, chunk_y, chunk_z), chunk);
        }
    }

    pub fn do_mesh_work(&mut self) {
        if let Some(mesh) = self.mesh_queue.pop() {
            if let Some(mut chunk) = self.chunks.remove(&mesh) {
                chunk.regenerate_mesh(&NeighboringTiles::new(&chunk, &self.chunks));

                self.chunks.insert(mesh, chunk);
            }
        }
    }

    pub fn cross_boundaries(&mut self, px: i32, py: i32, pz: i32) -> Result<(), Box<dyn Error>> {
        self.chunks.iter_mut().for_each(|(_, c)| { c.set_dirty(true) });

        for z in (-Self::RENDER_DISTANCE + pz)..=(Self::RENDER_DISTANCE + pz) {
            for y in (-Self::RENDER_DISTANCE + py)..=(Self::RENDER_DISTANCE + py) {
                for x in (-Self::RENDER_DISTANCE + px)..=(Self::RENDER_DISTANCE + px) {
                    let chunk_pos = (x, y, z);

                    if let Some(chunk) = self.chunks.get_mut(&chunk_pos) {
                        chunk.set_dirty(false);
                        if !self.mesh_queue.contains(&chunk_pos) {
                            self.mesh_queue.push(chunk_pos)
                        }
                    } else {
                        let chunk = Chunk::new(&self.gl, x, y, z)?;

                        self.mesh_queue.push((x, y, z));

                        self.chunks.insert((x, y, z), chunk);
                    }
                }
            }
        }

        self.chunks.retain(|_, chunk| { !chunk.dirty() });

        Ok(())
    }

    pub fn blit(&self) {
        for (_, chunk) in self.chunks.iter() {
            chunk.blit()
        }
    }
}
