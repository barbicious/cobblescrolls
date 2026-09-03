use crate::graphics::Bindable;
use glow::{HasContext, NativeVertexArray};
use std::error::Error;
use std::rc::Rc;

pub struct VertexArray {
    gl: Rc<glow::Context>,
    id: NativeVertexArray,
}

impl VertexArray {
    pub fn new(gl: &Rc<glow::Context>) -> Result<Self, Box<dyn Error>> {
        let gl = gl.clone();

        let id = unsafe { gl.create_vertex_array()? };

        Ok(Self { gl, id })
    }

    pub fn attr(&self, index: u32, size: i32, stride: i32, offset: i32) {
        unsafe {
            self.gl.enable_vertex_array_attrib(self.id, index);
            self.gl
                .vertex_attrib_pointer_f32(index, size, glow::FLOAT, false, stride, offset)
        }
    }
}

impl Bindable for VertexArray {
    fn bind(&self) {
        unsafe { self.gl.bind_vertex_array(Some(self.id)) }
    }

    fn unbind(&self) {
        unsafe { self.gl.bind_vertex_array(None) }
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe { self.gl.delete_vertex_array(self.id) }
    }
}
