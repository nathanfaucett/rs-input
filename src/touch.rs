use glutin::{self, TouchPhase};

use vec2;


pub struct Touch {
    id: u64,

    phase: TouchPhase,

    time_down: f64,
    time_up: f64,

    frame_down: usize,
    frame_up: usize,

    last: [f64; 2],

    position: [f64; 2],
    delta: [f64; 2],
}

impl Touch {
    pub fn new(touch: &glutin::Touch, current_time: f64, current_frame: usize) -> Self {
        let (x, y) = touch.location;

        Touch {
            id: touch.id,
            phase: touch.phase,

            time_down: current_time,
            time_up: current_time,

            frame_down: current_frame,
            frame_up: current_frame,

            last: [x, y],

            position: [x, y],
            delta: [0f64; 2],
        }
    }

    pub fn get_id(&self) -> u64 { self.id }
    pub fn get_phase(&self) -> TouchPhase { self.phase }

    pub fn get_time_down(&self) -> f64 { self.time_down }
    pub fn get_time_up(&self) -> f64 { self.time_up }

    pub fn get_frame_down(&self) -> usize { self.frame_down }
    pub fn get_frame_up(&self) -> usize { self.frame_up }

    pub fn get_position(&self) -> &[f64; 2] { &self.position }
    pub fn get_delta(&self) -> &[f64; 2] { &self.delta }

    pub fn update(&mut self, touch: &glutin::Touch, current_time: f64, current_frame: usize) {
        let (x, y) = touch.location;

        self.phase = touch.phase;

        match touch.phase {
            TouchPhase::Started => {
                self.frame_down = current_frame;
                self.time_down = current_time;
            },
            TouchPhase::Moved => (),
            TouchPhase::Ended => {
                self.frame_up = current_frame;
                self.time_up = current_time;
            },
            TouchPhase::Cancelled => {
                self.frame_up = current_frame;
                self.time_up = current_time;
            },
        }

        vec2::copy(&mut self.last, &self.position);
        vec2::set(&mut self.position, x, y);
        vec2::sub(&mut self.delta, &self.position, &self.last);
    }
}
