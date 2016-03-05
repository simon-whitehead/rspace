extern crate sdl2;

mod engine;
mod game;

fn main() {
    let mut window = engine::Window::new("RSpace", 800, 600);

    loop {
        if !window.process() {
            break;
        }

        window.set_scene(Box::new(game::scenes::GameScene));

        window.render();
    }
}
