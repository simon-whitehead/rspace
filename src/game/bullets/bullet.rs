extern crate sdl2;

use sdl2::render::Renderer;

pub trait Bullet {
    fn process(&mut self);
    fn render(&mut self, renderer: &mut Renderer);

    fn get_x(&self) -> i32;
    fn get_y(&self) -> i32;

    fn delete(&mut self);
    fn is_deleted(&self) -> bool;
}
