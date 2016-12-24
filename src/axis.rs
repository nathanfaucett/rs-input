use collections::string::String;

use clamp::Clamp;

use input::Input;


pub enum AxisType {
    Button,
    Mouse,
    Touch,
    Wheel,
    GamePad,
}


pub struct AxisOptions {
    pub name: String,

    pub neg_btn: String,
    pub pos_btn: String,

    pub alt_neg_btn: String,
    pub alt_pos_btn: String,

    pub gravity: f64,
    pub sensitivity: f64,

    pub dead: f64,

    pub axis_type: AxisType,
    pub axis: usize,

    pub index: usize,
    pub gamepad_index: usize,
}

impl Default for AxisOptions {
    fn default() -> Self {
        AxisOptions {
            name: String::new(),

            neg_btn: String::new(),
            pos_btn: String::new(),

            alt_neg_btn: String::new(),
            alt_pos_btn: String::new(),

            gravity: 3f64,
            sensitivity: 3f64,

            dead: 0.001f64,

            axis_type: AxisType::Button,
            axis: 0usize,

            index: 0usize,
            gamepad_index: 0usize,
        }
    }
}


pub struct Axis {
    name: String,

    neg_btn: String,
    pos_btn: String,

    alt_neg_btn: String,
    alt_pos_btn: String,

    gravity: f64,
    sensitivity: f64,

    dead: f64,

    axis_type: AxisType,
    axis: usize,

    index: usize,
    gamepad_index: usize,

    value: f64,
}

impl Axis {
    pub fn new(options: AxisOptions) -> Self {
        Axis {
            name: options.name,

            neg_btn: options.neg_btn,
            pos_btn: options.pos_btn,

            alt_neg_btn: options.alt_neg_btn,
            alt_pos_btn: options.alt_pos_btn,

            gravity: options.gravity,
            sensitivity: options.sensitivity,

            dead: options.dead,

            axis_type: options.axis_type,
            axis: options.axis,

            index: options.index,
            gamepad_index: options.gamepad_index,

            value: 0f64,
        }
    }

    pub fn get_name(&self) -> &String { &self.name }
    pub fn get_value(&self) -> f64 { self.value }

    pub fn update(&mut self, input: &mut Input, dt: f64) {
        let mut neg = false;
        let mut pos = false;
        let mut value = self.value;

        match self.axis_type {
            AxisType::Button => {
                neg =
                    input.get_button(&self.neg_btn).get_pressed() ||
                    input.get_button(&self.alt_neg_btn).get_pressed();
                pos =
                    input.get_button(&self.pos_btn).get_pressed() ||
                    input.get_button(&self.alt_pos_btn).get_pressed();
            },
            AxisType::Mouse => {
                self.value = input.get_mouse().get_delta()[self.axis] as f64;
                return;
            },
            AxisType::Touch => {
                if let Some(touch) = input.get_touches().get(self.index) {
                    self.value = touch.get_delta()[self.axis];
                }
                return;
            },
            AxisType::Wheel => (),
            AxisType::GamePad => (),
        }

        if neg {
            value -= self.sensitivity * dt;
        }
        if pos {
            value += self.sensitivity * dt;
        }

        if !pos && !neg && self.value != 0f64 {
            let tmp = value.abs();
            value -= (value.signum() * self.gravity * dt).clamp(-tmp, tmp);
        }

        value = value.clamp(-1f64, 1f64);
        if value.abs() <= self.dead {
            value = 0.0;
        }

        self.value = value;
    }
}
