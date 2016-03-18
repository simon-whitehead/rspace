
extern crate sdl2;
extern crate sdl2_image;
extern crate sdl2_ttf;

use engine::scene::Scene;
use engine::Window;

use game::scenes::GameScene;

mod engine;
mod game;

fn main() {
    let mut window = Window::new("RSpace", 800, 600);
    let scene = GameScene::new(window.width, window.height);

    window.set_scene(Box::new(scene));

    window.init();

    loop {
        if !window.process() {
            break;
        }

        window.render();
    }
}
