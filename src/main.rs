
extern crate sdl2;

use ::engine::scene::Scene;

mod engine;
mod game;

fn main() {
    let mut window = engine::Window::new("RSpace", 800, 600);
    let mut player = game::Player;
    let mut scene = game::scenes::GameScene::new();

    scene.add_entity(Box::new(player));

    window.set_scene(Box::new(scene));

    loop {
        if !window.process() {
            break;
        }

        window.render();
    }
}
