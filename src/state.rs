use crate::graphics::Bindable;
use crate::graphics::shader::Shader;
use crate::graphics::texture_atlas::TextureAtlas;
use crate::graphics::window::Window;
use crate::level::chunk::Chunk;
use glow::HasContext;
use nalgebra_glm::{Vec3, perspective};
use std::error::Error;
use std::rc::Rc;
use std::time::Instant;

pub struct State {
    window: Window,
    gl: Rc<glow::Context>,
}

impl State {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let (window, mut gl) = Window::new(
            1280,
            720,
            format!("cobblescrolls: v{}", env!("CARGO_PKG_VERSION")).as_str(),
        )?;

        unsafe {
            gl.enable(glow::DEPTH_TEST);
            gl.enable(glow::CULL_FACE)
        }

        Ok(Self { window, gl })
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let chunk = Chunk::new(&self.gl, 0, 0, 0)?;

        let shader = Shader::new(&self.gl, "res/shaders/cube.vert", "res/shaders/cube.frag")?;
        shader.bind();

        let proj = perspective(1280.0f32 / 720.0f32, 90.0_f32.to_radians(), 0.1, 1000.0);
        shader.set_mat4(proj, "proj");

        let texture_atlas = TextureAtlas::load(&self.gl, "res/cobblescrolls_atlas.png")?;
        texture_atlas.bind();

        let mut yaw = -90.0;
        let mut pitch = 0.0;

        let mut camera_pos: Vec3 = Vec3::new(0.0, 0.0, 0.0);
        let mut camera_front: Vec3 = Vec3::new(0.0, 0.0, -1.0);
        const CAMERA_UP: Vec3 = Vec3::new(0.0, 1.0, 0.0);
        const CAMERA_SPEED: f32 = 2.5;

        let mut last_tick = Instant::now();

        while self.window.good() {
            if self.window.is_key_down(glfw::Key::Escape) {
                break;
            }

            let now = Instant::now();
            let dt = (now - last_tick).as_secs_f32();
            last_tick = now;

            if self.window.is_key_down(glfw::Key::W) {
                camera_pos += (CAMERA_SPEED * dt) * camera_front;
            }

            if self.window.is_key_down(glfw::Key::S) {
                camera_pos -= (CAMERA_SPEED * dt) * camera_front;
            }

            if self.window.is_key_down(glfw::Key::A) {
                camera_pos -=
                    nalgebra_glm::normalize(&nalgebra_glm::cross(&camera_front, &CAMERA_UP))
                        * (CAMERA_SPEED * dt);
            }

            if self.window.is_key_down(glfw::Key::D) {
                camera_pos +=
                    nalgebra_glm::normalize(&nalgebra_glm::cross(&camera_front, &CAMERA_UP))
                        * (CAMERA_SPEED * dt);
            }

            if self.window.mouse().x_delta != 0.0 || self.window.mouse().y_delta != 0.0 {
                yaw += self.window.mouse().x_delta;
                pitch = (self.window.mouse().y_delta + pitch).clamp(-89.0, 89.0);

                camera_front = nalgebra_glm::normalize(&Vec3::new(
                    (yaw.to_radians().cos() * pitch.to_radians().cos()) as f32,
                    pitch.to_radians().sin() as f32,
                    (yaw.to_radians().sin() * pitch.to_radians().cos()) as f32,
                ))
            }

            shader.set_mat4(
                nalgebra_glm::look_at_rh(&camera_pos, &(camera_pos + camera_front), &CAMERA_UP),
                "view",
            );

            unsafe {
                self.gl.clear_color(0.3, 0.5, 0.8, 1.0);
                self.gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);

                chunk.blit();
            }

            self.window.display();
        }

        Ok(())
    }
}
