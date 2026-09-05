use crate::graphics::mesh::Mesh;
use crate::level::chunk::Chunk;
use crate::level::chunk::tiles::Tiles;
use crate::level::tile::{Face, TileType};
use crate::level::{NeighboringTiles, tile};
use glow::HasContext;
use std::error::Error;
use std::rc::Rc;

const TOTAL_VERTICES: usize = tile::TOTAL_VERTICES * Chunk::WIDTH * Chunk::HEIGHT * Chunk::DEPTH;

pub struct ChunkMesh {
    mesh: Mesh<TOTAL_VERTICES>,
    faces: usize,
    x: i32,
    y: i32,
    z: i32,
}

impl ChunkMesh {
    pub fn new(gl: &Rc<glow::Context>, x: i32, y: i32, z: i32) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            mesh: Mesh::new(gl)?,
            faces: 0,
            x,
            y,
            z,
        })
    }

    pub fn add_face(&mut self, face: Face, tile_type: TileType, x: i32, y: i32, z: i32) {
        self.mesh.upload(
            (self.faces * Face::VERTICES as usize)..(30 + self.faces * Face::VERTICES as usize),
            tile_type
                .vertices(
                    face,
                    x + self.x * Chunk::WIDTH as i32,
                    y + self.y * Chunk::HEIGHT as i32,
                    z + self.z * Chunk::DEPTH as i32,
                )
                .as_slice(),
        );
        self.faces += 1
    }

    pub fn regenerate_mesh(&mut self, tiles: &Tiles, neighboring_tiles: &NeighboringTiles) {
        self.faces = 0;
        self.mesh.clear();

        for z in 0..Chunk::DEPTH {
            for y in 0..Chunk::HEIGHT {
                for x in 0..Chunk::WIDTH {
                    let tile_type = tiles[Chunk::idx(x, y, z)];

                    if matches!(tile_type, TileType::Air) {
                        continue;
                    }

                    if tiles.is_tile_transparent(
                        x as i32,
                        y as i32,
                        z as i32 + 1,
                        neighboring_tiles,
                    ) {
                        self.add_face(Face::Front, tile_type, x as i32, y as i32, z as i32);
                    }

                    if tiles.is_tile_transparent(
                        x as i32,
                        y as i32,
                        z as i32 - 1,
                        neighboring_tiles,
                    ) {
                        self.add_face(Face::Back, tile_type, x as i32, y as i32, z as i32);
                    }

                    if tiles.is_tile_transparent(
                        x as i32,
                        y as i32 + 1,
                        z as i32,
                        neighboring_tiles,
                    ) {
                        self.add_face(Face::Top, tile_type, x as i32, y as i32, z as i32);
                    }

                    if tiles.is_tile_transparent(
                        x as i32,
                        y as i32 - 1,
                        z as i32,
                        neighboring_tiles,
                    ) {
                        self.add_face(Face::Bottom, tile_type, x as i32, y as i32, z as i32);
                    }

                    if tiles.is_tile_transparent(
                        x as i32 + 1,
                        y as i32,
                        z as i32,
                        neighboring_tiles,
                    ) {
                        self.add_face(Face::Right, tile_type, x as i32, y as i32, z as i32);
                    }

                    if tiles.is_tile_transparent(
                        x as i32 - 1,
                        y as i32,
                        z as i32,
                        neighboring_tiles,
                    ) {
                        self.add_face(Face::Left, tile_type, x as i32, y as i32, z as i32);
                    }
                }
            }
        }

        self.mesh.update();
    }

    pub fn blit(&self) {
        self.mesh.blit(self.faces as i32 * 6)
    }
}
