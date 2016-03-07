extern crate sdl2;

use ::engine::context::Context;

pub trait Entity {
    fn init(&mut self, renderer: &mut sdl2::render::Renderer);
    fn render(&mut self, renderer: &mut sdl2::render::Renderer, elapsed: f64);
    fn process(&mut self, events: &mut ::engine::events::Events, elapsed: f64);
}
