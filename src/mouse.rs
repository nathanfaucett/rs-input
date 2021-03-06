use glutin::Event;

use vec2;


#[derive(Debug)]
pub struct Mouse {
    first: bool,
    last: [isize; 2],

    focused: bool,

    position: [isize; 2],
    delta: [isize; 2],
}

impl Mouse {
    pub fn new() -> Self {
        Mouse {
            first: true,
            last: [0isize; 2],

            focused: true,

            position: [0isize; 2],
            delta: [0isize; 2],
        }
    }

    pub fn handle(&mut self, event: &Event) {
        match event {
            &Event::MouseEntered => {self.focused = true},
            &Event::MouseLeft => {self.focused = false},
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

    pub fn get_position(&self) -> &[isize; 2] { &self.position }
    pub fn get_delta(&self) -> &[isize; 2] { &self.delta }
}
