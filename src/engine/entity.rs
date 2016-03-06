extern crate sdl2;

use ::engine::context::Context;

pub trait Entity {
    fn render(&mut self, renderer: &mut sdl2::render::Renderer, events: & ::engine::events::Events, elapsed: f64);
    fn process(&mut self, events: & ::engine::events::Events, elapsed: f64);
}
