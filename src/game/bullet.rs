extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Renderer;

pub struct Bullet {
    pub deleted: bool,

    pub x: i32,
    pub y: i32
}

impl Bullet {
    pub fn new(position: (i32, i32)) -> Bullet {
        Bullet {
            deleted: false,
            x: position.0,
            y: position.1
        }
    }

    pub fn process(&mut self) {
        self.y = self.y - 10;

        if self.y < -10 {
            self.deleted = true;
        }
    }

    pub fn render(&mut self, renderer: &mut Renderer) {
        renderer.set_draw_color(Color::RGB(255, 255, 0));

        renderer.fill_rect(Rect::new(self.x, self.y, 2, 6));
    }
}
