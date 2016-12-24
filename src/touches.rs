use vector::Vector;
use stack::Stack;
use iterable_mut::IterableMut;
use collection::Collection;

use glutin::{Event, TouchPhase};

use touch::Touch;


pub struct Touches {
    vec: Vector<Touch>,
}

impl Touches {
    pub fn new() -> Self {
        Touches {
            vec: Vector::new(),
        }
    }

    fn find(&mut self, id: u64) -> Option<&mut Touch> {
        for touch in self.vec.iter_mut() {
            if touch.get_id() == id {
                return Some(touch);
            }
        }
        None
    }

    pub fn get(&self, index: usize) -> Option<&Touch> { self.vec.get(index) }
    pub fn len(&self) -> usize { self.vec.len() }

    pub fn reset(&mut self) {
        self.vec.retain(|touch|
            touch.get_phase() == TouchPhase::Cancelled ||
            touch.get_phase() == TouchPhase::Ended);
    }

    pub fn handle(&mut self, event: &Event, current_time: f64, current_frame: usize) {
        match event {
            &Event::Touch(t) => {
                let mut found = false;

                if let Some(touch) = self.find(t.id) {
                    touch.update(&t, current_time, current_frame);
                    found = true;
                }

                if !found {
                    self.vec.push(Touch::new(&t, current_time, current_frame));
                }
            },
            _ => (),
        }
    }
}
