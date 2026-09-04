use std::error::Error;
use std::ops::RangeBounds;
use std::rc::Rc;
use glow::HasContext;
use crate::graphics::Bindable;
use crate::graphics::buffer::vertex_buffer::VertexBuffer;
use crate::graphics::vertex_array::VertexArray;
use crate::level::chunk::Chunk;
use crate::level::tile::{Face, TOTAL_VERTICES};

pub struct Mesh<const N: usize> {
    vertices: [f32; N],
    vao: VertexArray,
    vbo: VertexBuffer,
    gl: Rc<glow::Context>,
}

impl<const N: usize> Mesh<N> {
    pub fn new(gl: &Rc<glow::Context>) -> Result<Self, Box<dyn Error>> {
        let gl = gl.clone();

        let vertices = [0_f32; N];

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
            gl,
        })
    }

    pub fn clear(&mut self) {
        self.vertices = [0_f32; N]
    }

    pub fn update(&mut self) {
        self.vao.bind();
        self.vbo.bind();
        self.vbo.submit_data::<f32>(self.vertices.as_slice());
    }

    pub fn upload<R>(&mut self, range: R, data: &[f32])
    where
        R: std::slice::SliceIndex<[f32], Output = [f32]>,
    {
        self.vertices[range].copy_from_slice(data);

    }

    pub fn blit(&self, count: i32) {
        unsafe {
            self.gl
                .draw_arrays(glow::TRIANGLES, 0, count);
        }
    }
}