use crate::graphics::buffer::Buffer;
use std::error::Error;
use std::ops::Deref;
use std::rc::Rc;

pub struct VertexBuffer {
    buffer: Buffer,
}

impl VertexBuffer {
    pub fn new(gl: &Rc<glow::Context>) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            buffer: Buffer::new(gl, glow::ARRAY_BUFFER)?,
        })
    }
}

impl Deref for VertexBuffer {
    type Target = Buffer;

    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}
