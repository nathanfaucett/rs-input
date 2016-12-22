use glutin::{Event, MouseButton, ElementState};

use vec2;


#[derive(Debug)]
pub struct Mouse {
    first: bool,
    last: [isize; 2],

    focused: bool,
    left: bool,
    right: bool,
    middle: bool,
    other: u8,

    position: [isize; 2],
    delta: [isize; 2],
}

impl Mouse {
    pub fn new() -> Self {
        Mouse {
            first: true,
            last: [0isize; 2],

            focused: true,
            left: false,
            right: false,
            middle: false,
            other: 0u8,

            position: [0isize; 2],
            delta: [0isize; 2],
        }
    }

    pub fn handle(&mut self, event: &Event) {
        match event {
            &Event::MouseEntered => {self.focused = true},
            &Event::MouseLeft => {self.focused = false},
            &Event::MouseInput(state, which) => {
                let value = match state {
                    ElementState::Released => false,
                    ElementState::Pressed => true,
                };

                match which {
                    MouseButton::Left => {self.left = value},
                    MouseButton::Right => {self.right = value},
                    MouseButton::Middle => {self.middle = value},
                    MouseButton::Other(index) => {self.other = index},
                }
            },
            &Event::MouseMoved(x, y) => {
                let x = x as isize;
                let y = y as isize;

                if self.first {
                    self.first = false;
                    vec2::set(&mut self.last, x, y);
                } else {
                    vec2::copy(&mut self.last, &self.position);
                }

                vec2::set(&mut self.position, x, y);
                vec2::sub(&mut self.delta, &self.position, &self.last);
            },
            _ => (),
        }
    }

    pub fn get_focused(&self) -> bool { self.focused }
    pub fn get_left(&self) -> bool { self.left }
    pub fn get_right(&self) -> bool { self.right }
    pub fn get_middle(&self) -> bool { self.middle }

    pub fn get_position(&self) -> &[isize; 2] { &self.position }
    pub fn get_delta(&self) -> &[isize; 2] { &self.delta }
}
