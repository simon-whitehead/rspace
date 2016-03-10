
extern crate sdl2;
extern crate sdl2_image;

use engine::entities::FrameAnimatedSprite;
use engine::scene::Scene;
use engine::Window;

use game::scenes::GameScene;
use game::Player;

mod engine;
mod game;

fn main() {
    let mut window = Window::new("RSpace", 800, 600);
    let mut scene = GameScene::new(window.width, window.height);
    let mut player = Player::new(scene.get_bounds());

    let cache_result = window.context.texture_cache.precache(&window.context.renderer, "assets/explosion/large/");

    for i in 0..5 {
        let entity = Box::new(FrameAnimatedSprite::new((i * 60, i * 60), 0.1, scene.get_bounds(), cache_result.clone()));
        scene.add_entity(entity);
    }

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
