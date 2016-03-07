
extern crate sdl2;
extern crate sdl2_image;

use ::engine::scene::Scene;

mod engine;
mod game;

fn main() {
    let mut window = engine::Window::new("RSpace", 800, 600);
    let mut player = game::Player::new();
    let mut scene = game::scenes::GameScene::new(window.width, window.height);

    player.set_bounds(scene.get_bounds());
    scene.add_entity(Box::new(player));

    window.set_scene(Box::new(scene));

    window.init();

    loop {
        if !window.process() {
            break;
        }

        window.render();
    }
}
