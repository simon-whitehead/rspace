
extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Renderer;

use ::engine::context::Context;

use ::game::bullets::Bullet;

pub struct MachineGunBullet {
    pub deleted: bool,

    pub x: i32,
    pub y: i32,

    player_owned: bool,  // Is this bullet owned by the player or an enemy?
    damage: i32
}

impl MachineGunBullet {
    pub fn new(position: (i32, i32)) -> MachineGunBullet {
        MachineGunBullet {
            deleted: false,
            x: position.0,
            y: position.1,
            player_owned: true,
            damage: 10
        }
    }

}

impl Bullet for MachineGunBullet {
    fn process(&mut self, _context: &mut Context) {
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

    fn is_player_owned(&self) -> bool {
        self.player_owned
    }

    fn damage(&self) -> i32 {
        self.damage
    }
}
