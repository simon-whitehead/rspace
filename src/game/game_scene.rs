extern crate sdl2;

use sdl2::pixels::Color;

use engine::context::Context;
use engine::scene::{Scene, SceneResult};

pub struct GameScene;

impl Scene for GameScene {
    fn render(&mut self, context: &mut Context, elapsed: f64) -> SceneResult {
        let renderer = &mut context.renderer;
        let events = &context.events;

        if events.quit || events.key_pressed(sdl2::keyboard::Keycode::Escape) {
            return SceneResult::Quit;
        }
        
        renderer.set_draw_color(Color::RGB(255, 0, 0));
        renderer.clear();
        renderer.present();

        SceneResult::None
    }
}
