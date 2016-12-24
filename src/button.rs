use collections::string::String;


pub struct Button {
    name: String,

    time_down: f64,
    time_up: f64,

    frame_down: usize,
    frame_up: usize,

    value: f64,
    pressed: bool,

    first: bool,
}

impl Button {
    pub fn new(name: String) -> Self {
        Button {
            name: name,

            time_down: -1f64,
            time_up: -1f64,

            frame_down: 0usize,
            frame_up: 0usize,

            value: 0.0f64,
            pressed: false,

            first: true,
        }
    }

    pub fn get_name(&self) -> &String { &self.name }

    pub fn get_time_down(&self) -> f64 { self.time_down }
    pub fn get_time_up(&self) -> f64 { self.time_up }

    pub fn get_frame_down(&self) -> usize { self.frame_down }
    pub fn get_frame_up(&self) -> usize { self.frame_up }

    pub fn get_value(&self) -> f64 { self.value }
    pub fn get_pressed(&self) -> bool { self.pressed }

    pub fn on(&mut self, value: f64, current_time: f64, current_frame: usize) {

        if self.first {
            self.first = false;
            self.frame_down = current_frame;
            self.time_down = current_time;
        }

        self.value = value;
        self.pressed = true;
    }
    pub fn off(&mut self, value: f64, current_time: f64, current_frame: usize) {

        self.frame_up = current_frame;
        self.time_up = current_time;

        self.value = value;
        self.pressed = false;

        self.first = true;
    }

    pub fn update(&mut self, value: f64, pressed: bool, current_time: f64, current_frame: usize) {
        if pressed {
            self.on(value, current_time, current_frame);
        } else {
            self.off(value, current_time, current_frame);
        }
    }
}
