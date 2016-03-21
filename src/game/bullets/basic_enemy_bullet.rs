
extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Renderer;

use ::engine::cache::{AssetCacheResult, TextureCache};
use ::engine::context::Context;

use ::game::bullets::Bullet;

pub struct BasicEnemyBullet {
    pub deleted: bool,

    pub x: i32,
    pub y: i32,

    pub width: u32,
    pub height: u32,

    cache: AssetCacheResult,
    bounds: Rect,
    
    player_owned: bool,
    damage: i32
}

impl BasicEnemyBullet {
    pub fn new(position: (i32, i32),
               size: (u32, u32),
               cache: AssetCacheResult,
               bounds: Rect) -> BasicEnemyBullet {

        BasicEnemyBullet {
            deleted: false,
            x: position.0,
            y: position.1,

            player_owned: false,

            damage: 10,

            width: size.0,
            height: size.1,
            
            cache: cache,
            bounds: bounds
        }
    }

}

impl Bullet for BasicEnemyBullet {
    fn process(&mut self, context: &mut Context) {
        self.y = self.y + 10;

        if self.y > context.bounds.height() as i32 + 10 {
            self.deleted = true;
        }
    }

    fn render(&mut self, texture_cache: &TextureCache, renderer: &mut Renderer) {
        let texture = &texture_cache.assets[self.cache.index as usize];

        renderer.copy(texture, Some(self.bounds), Some(Rect::new(self.x, self.y, self.width, self.height)));
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
