use glutin;

use window::Window;
use mouse::Mouse;
use button::Button;
use buttons::Buttons;
use touch::Touch;
use touches::Touches;
use axes::Axes;

use shared::Shared;


pub struct InputData {
    first: bool,
    last_time: f64,

    window: Window,
    mouse: Mouse,
    buttons: Buttons,
    touches: Touches,
    axes: Axes,
}

#[derive(Clone)]
pub struct Input {
    data: Shared<InputData>,
}

impl Input {
    pub fn new() -> Self {
        Input {
            data: Shared::new(InputData {
                first: true,
                last_time: 0f64,

                mouse: Mouse::new(),
                window: Window::new(),
                buttons: Buttons::new(),
                touches: Touches::new(),
                axes: Axes::new(),
            })
        }
    }

    pub fn get_window(&self) -> &Window { &self.data.window }
    pub fn get_mouse(&self) -> &Mouse { &self.data.mouse }
    pub fn get_touches(&self) -> &Touches { &self.data.touches }

    pub fn get_axis(&self, name: &str) -> f64 {
        match self.data.axes.get(name) {
            Some(axis) => axis.get_value(),
            None => 0f64,
        }
    }
    pub fn get_touch(&self, index: usize) -> Option<&Touch> { self.data.touches.get(index) }
    pub fn get_button(&mut self, name: &str) -> &Button { &self.data.buttons.get(name) }

    pub fn get_buttons(&self) -> &Buttons { &self.data.buttons }
    pub fn get_buttons_mut(&mut self) -> &mut Buttons { &mut self.data.buttons }

    pub fn get_axes(&self) -> &Axes { &self.data.axes }
    pub fn get_axes_mut(&mut self) -> &mut Axes { &mut self.data.axes }

    pub fn update(&mut self, window: &glutin::Window, current_time: f64, current_frame: usize) {
        {
            let ref mut data = self.data;

            data.window.reset();
            data.touches.reset();

            for event in window.poll_events() {
                data.mouse.handle(&event);
                data.window.handle(&event);
                data.touches.handle(&event, current_time, current_frame);
                data.buttons.handle(&event, current_time, current_frame);
            }

            if data.first {
                data.first = false;
                data.last_time = current_time;
            }
        }
        let mut input = self.clone();
        let ref mut data = self.data;
        let last_time = data.last_time;

        data.axes.update(&mut input, current_time - last_time);
        data.last_time = current_time;
    }
}
