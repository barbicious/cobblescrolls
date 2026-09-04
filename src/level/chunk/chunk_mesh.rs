use crate::graphics::Bindable;
use crate::graphics::buffer::vertex_buffer::VertexBuffer;
use crate::graphics::vertex_array::VertexArray;
use crate::level::chunk::Chunk;
use crate::level::tile::{Face, TOTAL_VERTICES, TileType};
use glow::HasContext;
use std::error::Error;
use std::rc::Rc;

pub(super) struct ChunkMesh {
    vertices: [f32; TOTAL_VERTICES * Chunk::WIDTH * Chunk::HEIGHT * Chunk::DEPTH],
    vao: VertexArray,
    vbo: VertexBuffer,
    faces: usize,
    gl: Rc<glow::Context>,
    x: i32, y: i32, z: i32,
}

impl ChunkMesh {
    pub fn new(gl: &Rc<glow::Context>, tiles: [TileType; Chunk::WIDTH * Chunk::HEIGHT * Chunk::DEPTH], x: i32, y: i32, z: i32) -> Result<Self, Box<dyn Error>> {
        let gl = gl.clone();

        let vertices = [0_f32; TOTAL_VERTICES * Chunk::WIDTH * Chunk::HEIGHT * Chunk::DEPTH];

        let vao = VertexArray::new(&gl)?;
        vao.bind();

        let vbo = VertexBuffer::new(&gl)?;
        vbo.bind();

        vbo.submit_data::<f32>(vertices.as_slice());

        vao.attr(0, 3, (5 * size_of::<f32>()) as i32, 0);
        vao.attr(
            1,
            2,
            (5 * size_of::<f32>()) as i32,
            (3 * size_of::<f32>()) as i32,
        );

        Ok(Self {
            vertices,
            vao,
            vbo,
            faces: 0,
            gl,
            x,
            y,
            z,
        })
    }

    pub fn add_face(&mut self, face: Face, tile_type: TileType, x: i32, y: i32, z: i32) {
        self.vertices[(self.faces * Face::VERTICES as usize)..(30 + self.faces * Face::VERTICES as usize)].copy_from_slice(tile_type.vertices(face, x, y, z).as_slice());
        self.faces += 1
    }

    pub fn regenerate_mesh(&mut self, tiles: &[TileType; Chunk::WIDTH * Chunk::HEIGHT * Chunk::DEPTH]) {
        self.faces = 0;
        self.vertices = [0_f32; TOTAL_VERTICES * Chunk::WIDTH * Chunk::HEIGHT * Chunk::DEPTH];

        for z in 0..Chunk::DEPTH {
            for y in 0..Chunk::HEIGHT {
                for x in 0..Chunk::WIDTH {
                    let tile_type = tiles[Chunk::idx(x, y, z)];

                    if matches!(tile_type, TileType::Air) {
                        continue
                    }

                    self.add_face(Face::Front, tile_type, x as i32, y as i32, z as i32);
                    self.add_face(Face::Back, tile_type, x as i32, y as i32, z as i32);
                    self.add_face(Face::Bottom, tile_type, x as i32, y as i32, z as i32);
                    self.add_face(Face::Left, tile_type, x as i32, y as i32, z as i32);
                    self.add_face(Face::Right, tile_type, x as i32, y as i32, z as i32);
                    self.add_face(Face::Top, tile_type, x as i32, y as i32, z as i32);
                }
            }
        }

        self.vao.bind();
        self.vbo.bind();
        self.vbo.submit_data::<f32>(self.vertices.as_slice());
    }

    pub fn blit(&self) {
        unsafe {
            self.gl.draw_arrays(glow::TRIANGLES, 0, (self.faces * 6) as i32);
        }
    }
}