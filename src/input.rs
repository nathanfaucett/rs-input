use glutin;

use window::Window;
use mouse::Mouse;


pub struct Input {
    window: Window,
    mouse: Mouse,
}

impl Input {
    pub fn new() -> Self {
        Input {
            mouse: Mouse::new(),
            window: Window::new(),
        }
    }

    pub fn get_window(&self) -> &Window { &self.window }
    pub fn get_mouse(&self) -> &Mouse { &self.mouse }

    pub fn update(&mut self, window: &glutin::Window, current_time: f64, current_frame: usize) {

        self.window.reset();

        for event in window.poll_events() {
            self.mouse.handle(&event);
            self.window.handle(&event);
        }
    }
}
