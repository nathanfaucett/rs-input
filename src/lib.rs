#![no_std]

extern crate glutin;

extern crate vec2;


mod input;
mod mouse;
mod window;


pub use input::Input;
pub use mouse::Mouse;
pub use window::Window;
