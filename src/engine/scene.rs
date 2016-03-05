extern crate sdl2;

use sdl2::pixels::Color;

use ::engine::context::Context;

pub enum SceneResult {
    None,
    Quit
}

pub trait Scene {
    fn render(&mut self, context: &mut Context, elapsed: f64) -> SceneResult;
}

pub struct DefaultScene;

impl Scene for DefaultScene {
    fn render(&mut self, context: &mut Context, elapsed: f64) -> SceneResult {
        let renderer = &mut context.renderer;
        let events = &context.events;

        if events.quit || events.key_pressed(sdl2::keyboard::Keycode::Escape) {
            return SceneResult::Quit;
        }

        
        renderer.set_draw_color(Color::RGB(0, 153, 204));
        renderer.clear();
        renderer.present();

        SceneResult::None
    }
}
