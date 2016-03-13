
extern crate sdl2;
extern crate sdl2_image;

use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, TextureQuery, Renderer};

use sdl2_image::LoadTexture;

use std::path::Path;

use ::engine::cache::TextureCache;
use ::engine::context::Context;
use ::engine::events::Events;

use ::game::bullet::Bullet;

pub struct BasicEnemy {
    pub x: i32,
    pub y: i32,

    pub width: u32,
    pub height: u32,

    pub dead: bool,
    pub health_points: u32,

    bounds: Rect,

    texture: Option<Texture>
}

impl BasicEnemy {
    pub fn new(position: (i32, i32),
               hp: u32,
               bounds: Rect) -> BasicEnemy {

        BasicEnemy {
            x: position.0,
            y: position.1,

            width: 0,
            height: 0,

            bounds: bounds,

            texture: None,

            health_points: hp,
            dead: false
        }
    }

    pub fn init(&mut self, context: &mut Context) {
        let tex = context.renderer.load_texture(Path::new("assets/enemies/basic.png")).unwrap();

        let TextureQuery { width, height, .. } = tex.query();

        self.width = width >> 1;
        self.height = height >> 1;

        self.texture = Some(tex);
    }

    pub fn render(&mut self, asset_cache: &TextureCache, renderer: &mut Renderer, elapsed: f64) {
        if let Some(ref tex) = self.texture {
            renderer.copy_ex(tex, Some(self.bounds), Some(Rect::new(self.x, self.y, self.width, self.height)), 0.0, Some(Point::new(self.width as i32 / 2, self.height as i32 / 2)), false, true);
        }
    }

    pub fn process(&mut self, events: &mut Events, elapsed: f64, time: u32) {
    }


    pub fn hit_test(&mut self, x: i32, y: i32) -> bool {
        x > self.x &&
        x < self.x + self.width as i32 &&
        y > self.y &&
        y < self.y + self.height as i32
    }

    pub fn is_dead(&self) -> bool {
        self.health_points <= 0
    }
}
