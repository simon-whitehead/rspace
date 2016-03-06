extern crate sdl2;

use sdl2::pixels::Color;

use ::engine::context::Context;
use ::engine::entity::Entity;
use ::engine::scene::{SceneResult};

pub struct Player;

impl Entity for Player {
    fn render(&mut self, renderer: &mut sdl2::render::Renderer, events: & ::engine::events::Events, elapsed: f64) {
        renderer.set_draw_color(Color::RGB(255, 255, 255));

        renderer.fill_rect(sdl2::rect::Rect::new(100, 100, 50, 50));
    }

    fn process(&mut self, events: & ::engine::events::Events, elapsed: f64) {
    }
}
