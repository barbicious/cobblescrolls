use std::error::Error;
use std::rc::Rc;
use glow::HasContext;
use crate::graphics::Bindable;
use crate::graphics::buffer::vertex_buffer::VertexBuffer;
use crate::graphics::vertex_array::VertexArray;
use crate::level::chunk::Chunk;
use crate::level::tile::{TileType, TOTAL_VERTICES, Face};

pub(super) struct ChunkMesh {
    vertices: [f32; TOTAL_VERTICES * Chunk::WIDTH * Chunk::HEIGHT * Chunk::DEPTH],
    vao: VertexArray,
    vbo: VertexBuffer,
    faces: usize,
    gl: Rc<glow::Context>
}

impl ChunkMesh {
    pub fn new(gl: &Rc<glow::Context>, x: i32, y: i32, z: i32) -> Result<Self, Box<dyn Error>> {
        let gl = gl.clone();

        let mut vertices = [0_f32; TOTAL_VERTICES * Chunk::WIDTH * Chunk::HEIGHT * Chunk::DEPTH];

        let mut i = 0;
        for z in 0..Chunk::DEPTH {
            for y in 0..Chunk::HEIGHT {
                for x in 0..Chunk::WIDTH {
                    vertices[(0 + i * 180)..(30 + i * 180)].copy_from_slice(TileType::Grass.vertices(Face::Front, x as i32, y as i32, z as i32).as_slice());
                    vertices[(30 + i * 180)..(60 + i * 180)].copy_from_slice(TileType::Grass.vertices(Face::Back, x as i32, y as i32, z as i32).as_slice());
                    vertices[(60 + i * 180)..(90 + i * 180)].copy_from_slice(TileType::Grass.vertices(Face::Bottom, x as i32, y as i32, z as i32).as_slice());
                    vertices[(90 + i * 180)..(120 + i * 180)].copy_from_slice(TileType::Grass.vertices(Face::Left, x as i32, y as i32, z as i32).as_slice());
                    vertices[(120 + i * 180)..(150 + i * 180)].copy_from_slice(TileType::Grass.vertices(Face::Top, x as i32, y as i32, z as i32).as_slice());
                    vertices[(150 + i * 180)..(180 + i * 180)].copy_from_slice(TileType::Grass.vertices(Face::Right, x as i32, y as i32, z as i32).as_slice());
                    i += 1;
                }
            }
        }

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
            faces: 6 * i,
            gl
        })
    }

    pub fn blit(&self) {
        unsafe {
            self.gl.draw_arrays(glow::TRIANGLES, 0, (self.faces * 6) as i32);
        }
    }
}