pub mod buffer;
pub mod shader;
pub mod texture_atlas;
pub mod vertex_array;
pub mod window;
pub mod mesh;

pub trait Bindable {
    fn bind(&self);
    fn unbind(&self);
}
