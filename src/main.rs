extern crate sdl2;

mod engine;

fn main() {
    let mut window = engine::Window::new("RSpace", 800, 600);


    loop {
        if !window.process() {
            break;
        }

        window.render();
    }
}
