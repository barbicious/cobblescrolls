pub mod index_buffer;
pub mod vertex_buffer;

use crate::graphics::Bindable;
use glow::{HasContext, NativeBuffer};
use std::error::Error;
use std::rc::Rc;

pub struct Buffer {
    gl: Rc<glow::Context>,
    id: NativeBuffer,
    target: u32,
}

impl Buffer {
    pub fn new(gl: &Rc<glow::Context>, target: u32) -> Result<Self, Box<dyn Error>> {
        let gl = gl.clone();

        let id = unsafe { gl.create_buffer()? };

        Ok(Self { target, id, gl })
    }

    pub fn submit_data<T>(&self, data: &[T])
    where
        T: bytemuck::Pod + bytemuck::Zeroable,
    {
        unsafe {
            self.gl
                .buffer_data_u8_slice(self.target, bytemuck::cast_slice(data), glow::STATIC_DRAW)
        }
    }
}

impl Bindable for Buffer {
    fn bind(&self) {
        unsafe { self.gl.bind_buffer(self.target, Some(self.id)) }
    }

    fn unbind(&self) {
        unsafe { self.gl.bind_buffer(self.target, None) }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe { self.gl.delete_buffer(self.id) }
    }
}
