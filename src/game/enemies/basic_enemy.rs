
extern crate sdl2;
extern crate sdl2_image;

use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, TextureQuery, Renderer};

use sdl2_image::LoadTexture;

use std::path::Path;

use ::engine::cache::{AssetCacheResult, TextureCache};
use ::engine::context::Context;
use ::engine::entities::FrameAnimatedSprite;
use ::engine::events::Events;

use ::game::bullet::Bullet;
use ::game::enemies::Enemy;
use ::game::explosion::Explosion;

pub struct BasicEnemy {
    pub x: i32,
    pub y: i32,

    pub width: u32,
    pub height: u32,

    pub dead: bool,
    pub health_points: i32,

    bounds: Rect,

    texture: Option<Texture>,

    explosion_cache: AssetCacheResult,

    move_interval: u32,
    last_move_time: u32
}

impl BasicEnemy {
    pub fn new(position: (i32, i32),
               hp: i32,
               bounds: Rect,
               explosion_cache: AssetCacheResult) -> BasicEnemy {

        BasicEnemy {
            x: position.0,
            y: position.1,

            width: 0,
            height: 0,

            bounds: bounds,

            texture: None,

            explosion_cache: explosion_cache,

            health_points: hp,
            dead: false,

            move_interval: 50,   // Every 50 milliseconds, move down the screen slightly
            last_move_time: 0
        }
    }
}

impl Enemy for BasicEnemy {
    fn init(&mut self, context: &mut Context) {
        let tex = context.renderer.load_texture(Path::new("assets/enemies/basic.png")).unwrap();

        let TextureQuery { width, height, .. } = tex.query();

        self.width = width >> 1;
        self.height = height >> 1;

        self.texture = Some(tex);
    }

    fn render(&mut self, asset_cache: &TextureCache, renderer: &mut Renderer, elapsed: f64) {
        if let Some(ref tex) = self.texture {
            renderer.copy_ex(tex, Some(self.bounds), Some(Rect::new(self.x, self.y, self.width, self.height)), 0.0, Some(Point::new(self.width as i32 / 2, self.height as i32 / 2)), false, true);
        }
    }

    fn process(&mut self, events: &mut Events, elapsed: f64, time: u32) {
        // Should we move down slightly?
        if time - self.last_move_time >= self.move_interval {
            self.y += 1;

            self.last_move_time = time;
        }
    }

    fn hit_test(&mut self, rect: sdl2::rect::Rect) -> bool {
        let enemy_rect = Rect::new(self.x, self.y, self.width, self.height);

        ::engine::helpers::overlap(enemy_rect, rect)
    }

    fn is_dead(&self) -> bool {
        self.health_points <= 0
    }

    fn take_damage(&mut self, damage: i32) {
        self.health_points -= damage;
    }

    fn get_x(&self) -> i32 {
        self.x
    }

    fn get_y(&self) -> i32 {
        self.y
    }

    fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    fn set_y(&mut self, y: i32) {
        self.y = y;
    }

    fn get_width(&self) -> u32 {
        self.width
    }

    fn get_height(&self) -> u32 {
        self.height
    }

    // Generate explosions for this enemy
    fn explode(&self, context: &mut Context) -> Vec<Explosion> {
        let mut explosions = Vec::new();

        let bounds = self.bounds;
        let start_x = self.x + (self.width as i32 / 2);
        let start_y = self.y + (self.height as i32 / 2);

        let mut sprite = FrameAnimatedSprite::new(0.05, bounds, self.explosion_cache.clone());
        sprite.init(context);

        let x = start_x - sprite.width as i32 / 2;
        let y = start_y - (sprite.height as i32 / 2);
        explosions.push(Explosion::new((x, y), sprite));

        explosions
    }
}
