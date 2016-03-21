extern crate sdl2;

use sdl2::render::Renderer;

use ::engine::cache::TextureCache;
use ::engine::context::Context;

pub trait Bullet {
    fn process(&mut self, context: &mut Context);
    fn render(&mut self, texture_cache: &TextureCache, renderer: &mut Renderer);

    fn get_x(&self) -> i32;
    fn get_y(&self) -> i32;

    fn delete(&mut self);
    fn is_deleted(&self) -> bool;

    fn is_player_owned(&self) -> bool;

    fn damage(&self) -> i32;
}
