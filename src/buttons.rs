use collections::string::String;
use core::fmt;

use glutin::{Event, ElementState, MouseButton};

use vector::Vector;
use stack::Stack;

use hash_map::HashMap;
use map::Map;
use insert::Insert;

use button::Button;


pub struct Buttons {
    vec: Vector<Button>,
    map: HashMap<String, Button>,
}

impl Buttons {
    pub fn new() -> Self {
        Buttons {
            vec: Vector::new(),
            map: HashMap::new(),
        }
    }

    pub fn handle(&mut self, event: &Event, current_time: f64, current_frame: usize) {
        match event {
            &Event::MouseInput(state, which) => {
                let (value, pressed) = match state {
                    ElementState::Released => (0f64, false),
                    ElementState::Pressed => (1f64, true),
                };

                match which {
                    MouseButton::Left => self.update(String::from("mouse0"), value, pressed, current_time, current_frame),
                    MouseButton::Right => self.update(String::from("mouse1"), value, pressed, current_time, current_frame),
                    MouseButton::Middle => self.update(String::from("mouse2"), value, pressed, current_time, current_frame),
                    MouseButton::Other(_) => {},
                }
            },
            &Event::KeyboardInput(state, _, key_code) => {
                if let Some(code) = key_code {
                    let mut name = String::new();

                    fmt::write(&mut name, format_args!("{:?}", code))
                        .expect("Error occurred while trying to write in String");

                    match state {
                        ElementState::Released => {
                            self.off(name, 0f64, current_time, current_frame);
                        },
                        ElementState::Pressed => {
                            self.on(name, 1f64, current_time, current_frame);
                        },
                    };
                }
            },
            _ => {},
        }
    }

    pub fn get(&mut self, name: &str) -> &Button {
        let n = String::from(name);
        if !self.map.contains_key(&n) {
            self.add_button(n.clone());
        }
        self.map.get_mut(&n).unwrap()
    }

    pub fn on(&mut self, name: String, value: f64, current_time: f64, current_frame: usize) {
        if !self.map.contains_key(&name) {
            self.add_button(name.clone());
        }
        self.map.get_mut(&name).unwrap().on(value, current_time, current_frame);
    }
    pub fn off(&mut self, name: String, value: f64, current_time: f64, current_frame: usize) {
        if !self.map.contains_key(&name) {
            self.add_button(name.clone());
        }
        self.map.get_mut(&name).unwrap().off(value, current_time, current_frame);
    }
    pub fn update(&mut self, name: String, value: f64, pressed: bool, current_time: f64, current_frame: usize) {
        if !self.map.contains_key(&name) {
            self.add_button(name.clone());
        }
        self.map.get_mut(&name).unwrap().update(value, pressed, current_time, current_frame);
    }

    fn add_button(&mut self, name: String) {
        let button = Button::new(name.clone());
        self.vec.push(button.clone());
        self.map.insert(name, button);
    }
}
