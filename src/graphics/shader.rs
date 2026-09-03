use crate::graphics::Bindable;
use glow::{HasContext, NativeShader, Program};
use nalgebra_glm::Mat4;
use std::error::Error;
use std::path::Path;
use std::rc::Rc;

pub struct Shader {
    gl: Rc<glow::Context>,
    id: Program,
}

impl Shader {
    pub fn new(
        gl: &Rc<glow::Context>,
        vs_path: &str,
        fs_path: &str,
    ) -> Result<Self, Box<dyn Error>> {
        let gl = gl.clone();

        let vs_shader = Self::load_shader(&gl, vs_path, glow::VERTEX_SHADER)?;
        let fs_shader = Self::load_shader(&gl, fs_path, glow::FRAGMENT_SHADER)?;

        let id = unsafe { gl.create_program()? };

        unsafe {
            gl.attach_shader(id, vs_shader);
            gl.attach_shader(id, fs_shader);

            gl.link_program(id);

            if !gl.get_program_link_status(id) {
                return Err(Box::from(gl.get_program_info_log(id)));
            }

            gl.delete_shader(vs_shader);
            gl.delete_shader(fs_shader);
        }

        Ok(Self { gl, id })
    }

    fn load_shader(
        gl: &Rc<glow::Context>,
        path: impl AsRef<Path>,
        r#type: u32,
    ) -> Result<NativeShader, Box<dyn Error>> {
        let shader = unsafe { gl.create_shader(r#type)? };

        let src = std::fs::read_to_string(path)?;

        unsafe {
            gl.shader_source(shader, src.as_str());
            gl.compile_shader(shader);
        }

        unsafe {
            if !gl.get_shader_compile_status(shader) {
                return Err(Box::from(gl.get_shader_info_log(shader)));
            }
        }

        Ok(shader)
    }

    pub fn set_mat4(&self, mat4: Mat4, name: &str) {
        unsafe {
            self.gl.uniform_matrix_4_f32_slice(
                self.gl.get_uniform_location(self.id, name).as_ref(),
                false,
                mat4.as_ref(),
            );
        }
    }
}

impl Bindable for Shader {
    fn bind(&self) {
        unsafe { self.gl.use_program(Some(self.id)) }
    }

    fn unbind(&self) {
        unsafe { self.gl.use_program(None) }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { self.gl.delete_program(self.id) }
    }
}
