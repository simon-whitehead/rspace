extern crate sdl2;

use sdl2::rect::Rect;
use sdl2::render::Renderer;

use ::engine::cache::{AssetCacheResult, TextureCache};
use ::engine::context::Context;
use ::engine::events::Events;

use ::game::explosion::Explosion;

pub trait Enemy {
    fn init(&mut self, context: &mut Context);
    fn render(&mut self, asset_cache: &TextureCache, renderer: &mut Renderer, elapsed: f64);
    fn process(&mut self, events: &mut Events, elapsed: f64, time: u32);
    fn hit_test(&mut self, rect: sdl2::rect::Rect) -> bool;
    fn is_dead(&self) -> bool;
    fn take_damage(&mut self, damage: i32);

    fn get_x(&self) -> i32;
    fn get_y(&self) -> i32;

    fn get_width(&self) -> u32;
    fn get_height(&self) -> u32;

    fn explode(&self, context: &mut Context) -> Vec<Explosion>;
}