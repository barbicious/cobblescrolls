use crate::graphics::Bindable;
use glow::{HasContext, Texture};
use std::error::Error;
use std::path::Path;
use std::rc::Rc;

pub struct TextureAtlas {
    gl: Rc<glow::Context>,
    id: Texture,
}

impl TextureAtlas {
    pub const WIDTH: f32 = 256.0;
    pub const HEIGHT: f32 = 256.0;

    pub fn load(gl: &Rc<glow::Context>, path: impl AsRef<Path>) -> Result<Self, Box<dyn Error>> {
        let gl = gl.clone();

        let id = unsafe { gl.create_texture()? };

        unsafe {
            gl.bind_texture(glow::TEXTURE_2D, Some(id));

            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::REPEAT as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::REPEAT as i32);
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MIN_FILTER,
                glow::NEAREST as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MAG_FILTER,
                glow::NEAREST as i32,
            );
        }

        let image = image::open(path)?.flipv();

        unsafe {
            gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                glow::RGBA as i32,
                Self::WIDTH as i32,
                Self::HEIGHT as i32,
                0,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                glow::PixelUnpackData::Slice(Some(image.as_bytes())),
            );
        }

        Ok(Self { gl, id })
    }
}

impl Bindable for TextureAtlas {
    fn bind(&self) {
        unsafe { self.gl.bind_texture(glow::TEXTURE_2D, Some(self.id)) }
    }

    fn unbind(&self) {
        unsafe { self.gl.bind_texture(glow::TEXTURE_2D, None) }
    }
}
