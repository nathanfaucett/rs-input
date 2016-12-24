extern crate gl;
extern crate glutin;

extern crate input;


use input::Input;


fn main() {
    let window = glutin::Window::new().unwrap();
    let mut input = Input::new();

    unsafe {
        match window.make_current() {
            Ok(_) => {
                gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
            },
            Err(e) => panic!("{:?}", e),
        }

        gl::ClearColor(0.3f32, 0.3f32, 0.3f32, 1.0f32);
    }

    let mut time = 0f64;
    let mut frame = 0usize;
    let mut movement = [0f32; 2];
    while !input.get_window().get_closed() {

        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT); }

        input.update(&window, time, frame);

        let w = input.get_window();
        if w.get_resized() {
            println!("Resized {:?} {:?}", w.get_width(), w.get_height());
        }

        let x = input.get_axis("Horizontal") as f32;
        let y = input.get_axis("Vertical") as f32;
        if movement[0] != x || movement[1] != y {
            movement[0] = x;
            movement[1] = y;
            println!("{:?}", movement);
        }

        time += 1f64 / 60f64;
        frame += 1usize;

        match window.swap_buffers() {
            Ok(_) => (),
            Err(e) => panic!("{:?}", e),
        }
    }
}
