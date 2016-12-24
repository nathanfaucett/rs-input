#![feature(collections)]
#![no_std]


extern crate collections;


extern crate glutin;

extern crate shared;

extern crate clamp;

extern crate vector;
extern crate stack;
extern crate iterable;
extern crate collection;

extern crate hash_map;
extern crate map;
extern crate insert;
extern crate iterable_mut;

extern crate vec2;


mod axes;
mod axis;
mod button;
mod buttons;
mod input;
mod mouse;
mod touch;
mod touches;
mod window;


pub use axes::Axes;
pub use axis::{Axis, AxisOptions, AxisType};
pub use button::Button;
pub use buttons::Buttons;
pub use input::Input;
pub use mouse::Mouse;
pub use touch::Touch;
pub use touches::Touches;
pub use window::Window;
