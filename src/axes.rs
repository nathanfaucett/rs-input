use collections::string::String;

use hash_map::HashMap;
use map::Map;
use insert::Insert;
use iterable_mut::IterableMut;

use input::Input;
use axis::{Axis, AxisOptions};


pub struct Axes {
    map: HashMap<String, Axis>,
}

impl Axes {
    pub fn new() -> Self {
        let mut axes = Axes {
            map: HashMap::new(),
        };

        let mut options = AxisOptions::default();
        options.name = String::from("Horizontal");
        options.pos_btn = String::from("Right");
        options.neg_btn = String::from("Left");
        options.alt_pos_btn = String::from("D");
        options.alt_neg_btn = String::from("A");
        axes.add(Axis::new(options));

        let mut options = AxisOptions::default();
        options.name = String::from("Vertical");
        options.pos_btn = String::from("Up");
        options.neg_btn = String::from("Down");
        options.alt_pos_btn = String::from("W");
        options.alt_neg_btn = String::from("S");
        axes.add(Axis::new(options));

        let mut options = AxisOptions::default();
        options.name = String::from("Jump");
        options.pos_btn = String::from("Space");
        options.neg_btn = String::from("");
        options.alt_pos_btn = String::from("Mouse2");
        options.alt_neg_btn = String::from("");
        axes.add(Axis::new(options));

        axes
    }

    pub fn add(&mut self, axis: Axis) {
        let name = axis.get_name().clone();
        self.map.insert(name, axis);
    }

    pub fn has(&self, name: &str) -> bool { self.map.contains_key(&String::from(name)) }
    pub fn get(&self, name: &str) -> Option<&Axis> { self.map.get(&String::from(name)) }

    pub fn update(&mut self, input: &mut Input, dt: f64) {
        for (_, axis) in self.map.iter_mut() {
            axis.update(input, dt);
        }
    }
}
