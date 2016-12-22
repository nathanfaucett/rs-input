use collections::string::String;
use shared::Shared;


pub struct ButtonData {
    name: String,

    time_down: f64,
    time_up: f64,

    frame_down: usize,
    frame_up: usize,

    value: f64,
    pressed: bool,

    first: bool,
}

#[derive(Clone)]
pub struct Button {
    data: Shared<ButtonData>,
}

impl Button {
    pub fn new(name: String) -> Self {
        Button {
            data: Shared::new(ButtonData {
                name: name,

                time_down: -1f64,
                time_up: -1f64,

                frame_down: 0usize,
                frame_up: 0usize,

                value: 0.0f64,
                pressed: false,

                first: true,
            })
        }
    }

    pub fn get_name(&self) -> &String { &self.data.name }

    pub fn get_time_down(&self) -> f64 { self.data.time_down }
    pub fn get_time_up(&self) -> f64 { self.data.time_up }

    pub fn get_frame_down(&self) -> usize { self.data.frame_down }
    pub fn get_frame_up(&self) -> usize { self.data.frame_up }

    pub fn get_value(&self) -> f64 { self.data.value }
    pub fn get_pressed(&self) -> bool { self.data.pressed }

    pub fn on(&mut self, value: f64, current_time: f64, current_frame: usize) {
        let ref mut data = self.data;

        if data.first {
            data.first = false;
            data.frame_down = current_frame;
            data.time_down = current_time;
        }

        data.value = value;
        data.pressed = true;
    }
    pub fn off(&mut self, value: f64, current_time: f64, current_frame: usize) {
        let ref mut data = self.data;

        data.frame_up = current_frame;
        data.time_up = current_time;

        data.value = value;
        data.pressed = false;

        data.first = true;
    }

    pub fn update(&mut self, value: f64, pressed: bool, current_time: f64, current_frame: usize) {
        if pressed {
            self.on(value, current_time, current_frame);
        } else {
            self.off(value, current_time, current_frame);
        }
    }
}
