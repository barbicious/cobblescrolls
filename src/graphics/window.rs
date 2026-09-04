use glfw::{fail_on_errors, Context, Glfw, GlfwReceiver, Key, MouseButton, PWindow};
use glow::HasContext;
use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashSet;
use std::error::Error;
use std::rc::Rc;

pub struct Mouse {
    pub x: f64,
    pub y: f64,
    pub x_delta: f64,
    pub y_delta: f64,
    first: bool,
    previous_buttons: HashSet<MouseButton>,
    current_buttons: HashSet<MouseButton>
}

impl Mouse {
    const SENSITIVITY: f64 = 0.1;
}

pub struct Keyboard {
    previous_keys: HashSet<Key>,
    current_keys: HashSet<Key>,
}

pub struct Window {
    p_window: PWindow,
    glfw: Glfw,
    event: GlfwReceiver<(f64, glfw::WindowEvent)>,
    mouse: Rc<RefCell<Mouse>>,
    keyboard: Keyboard,
}

impl Window {
    pub fn new(
        width: u32,
        height: u32,
        title: &str,
    ) -> Result<(Self, Rc<glow::Context>), Box<dyn Error>> {
        let mut glfw = glfw::init(fail_on_errors!())?;

        let (mut p_window, event) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .ok_or_else(|| "Failed to create window")?;

        p_window.make_current();

        let glow_context = Rc::new(unsafe {
            glow::Context::from_loader_function(|s| {
                if let Some(proc) = glfw.get_proc_address_raw(s) {
                    proc as *const std::ffi::c_void
                } else {
                    std::ptr::null()
                }
            })
        });

        let framebuffer_size_cb_gl = glow_context.clone();
        p_window.set_framebuffer_size_callback(move |_window, w, h| unsafe {
            framebuffer_size_cb_gl.viewport(0, 0, w, h)
        });

        let mouse = Rc::new(RefCell::new(Mouse {
            x: 0.0,
            y: 0.0,
            first: true,
            previous_buttons: HashSet::with_capacity(MouseButton::Button8 as usize),
            x_delta: 0.0,
            y_delta: 0.0,
            current_buttons: HashSet::with_capacity(MouseButton::Button8 as usize),
        }));

        let cursor_pos_callback_mouse = mouse.clone();

        p_window.set_cursor_pos_callback(move |_window, x, y| {
            let mut mouse_cb = cursor_pos_callback_mouse.borrow_mut();

            if mouse_cb.first {
                mouse_cb.x = x;
                mouse_cb.y = y;
                mouse_cb.first = false;
            }

            mouse_cb.x_delta = (x - mouse_cb.x) * Mouse::SENSITIVITY;
            mouse_cb.y_delta = (mouse_cb.y - y) * Mouse::SENSITIVITY;

            mouse_cb.x = x;
            mouse_cb.y = y;
        });

        p_window.set_cursor_mode(glfw::CursorMode::Disabled);

        p_window.set_all_polling(true);

        Ok((
            Self {
                p_window,
                event,
                glfw,
                mouse,
                keyboard: Keyboard {
                    current_keys: HashSet::with_capacity(Key::Menu as usize),
                    previous_keys: HashSet::with_capacity(Key::Menu as usize),
                },
            },
            glow_context,
        ))
    }

    pub fn mouse(&self) -> Ref<'_, Mouse> {
        self.mouse.borrow()
    }

    pub fn good(&mut self) -> bool {
        self.glfw.poll_events();

        self.keyboard.previous_keys = self.keyboard.current_keys.clone();

        let mut mouse = self.mouse.borrow_mut();

        mouse.previous_buttons = mouse.current_buttons.clone();

        for (_, event) in glfw::flush_messages(&self.event) {
            match event {
                glfw::WindowEvent::Key(key, _, glfw::Action::Press, _) => {
                    self.keyboard.current_keys.insert(key);
                }
                glfw::WindowEvent::Key(key, _, glfw::Action::Release, _) => {
                    self.keyboard.current_keys.remove(&key);
                }
                glfw::WindowEvent::MouseButton(button, glfw::Action::Press, _) => {
                    mouse.current_buttons.insert(button);
                }
                glfw::WindowEvent::MouseButton(button, glfw::Action::Release, _) => {
                    mouse.current_buttons.remove(&button);
                }
                _ => {}
            }
        }

        !self.p_window.should_close()
    }

    pub fn is_key_down(&self, key: Key) -> bool {
        self.keyboard.previous_keys.contains(&key) && self.keyboard.current_keys.contains(&key)
    }

    pub fn is_mouse_down(&self, mouse_button: MouseButton) -> bool {
        self.mouse().current_buttons.contains(&mouse_button) && self.mouse().previous_buttons.contains(&mouse_button)
    }

    pub fn display(&mut self) {
        self.p_window.swap_buffers();
        let mut mouse = self.mouse.borrow_mut();
        mouse.x_delta = 0.0;
        mouse.y_delta = 0.0;
    }
}
