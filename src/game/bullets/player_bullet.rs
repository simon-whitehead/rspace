
extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Renderer;

use ::game::bullets::Bullet;

pub struct PlayerBullet {
    pub deleted: bool,

    pub x: i32,
    pub y: i32
}

impl PlayerBullet {
    pub fn new(position: (i32, i32)) -> PlayerBullet {
        PlayerBullet {
            deleted: false,
            x: position.0,
            y: position.1
        }
    }

}

impl Bullet for PlayerBullet {
    fn process(&mut self) {
        self.y = self.y - 10;

        if self.y < -10 {
            self.deleted = true;
        }
    }

    fn render(&mut self, renderer: &mut Renderer) {
        renderer.set_draw_color(Color::RGB(255, 255, 0));

        renderer.fill_rect(Rect::new(self.x, self.y, 2, 6)).unwrap();
    }

    fn get_x(&self) -> i32 {
        self.x
    }

    fn get_y(&self) -> i32 {
        self.y
    }

    fn delete(&mut self) {
        self.deleted = true;
    }

    fn is_deleted(&self) -> bool {
        self.deleted 
    }
}
