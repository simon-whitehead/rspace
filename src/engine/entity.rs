extern crate sdl2;

use sdl2::render::Renderer;

use ::engine::cache::TextureCache;
use ::engine::context::Context;
use ::engine::events::Events;

pub trait Entity {
    fn init(&mut self, context: &mut Context);
    fn render(&mut self, texture_cache: &TextureCache, renderer: &mut Renderer, elapsed: f64);
    fn process(&mut self, event_handler: &mut Events, elapsed: f64);
}
