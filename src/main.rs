
extern crate sdl2;
extern crate sdl2_image;
extern crate sdl2_ttf;

use std::path::Path;
use sdl2::pixels::Color;

use ::engine::scene::Scene;

mod engine;
mod game;

fn main() {
    let mut window = engine::Window::new("RSpace", 800, 600);
    let mut scene = game::scenes::GameScene::new(window.width, window.height);
    let mut player = game::Player::new(scene.get_bounds());

    let mut explosion = engine::entities::FrameAnimatedSprite::new(Path::new("assets/explosion/large/"), 0.1, scene.get_bounds());
    
    scene.add_entity(Box::new(player));
    scene.add_entity(Box::new(explosion));

    window.set_scene(Box::new(scene));

    window.init();

    loop {
        if !window.process() {
            break;
        }

        window.render();
    }
}
