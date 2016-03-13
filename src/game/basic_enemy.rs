
extern crate sdl2;
extern crate sdl2_image;

use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureQuery, Renderer};

use sdl2_image::LoadTexture;

use std::path::Path;

use ::engine::cache::TextureCache;
use ::engine::context::Context;
use ::engine::events::Events;

pub struct BasicEnemy {
    x: i32,
    y: i32,

    width: u32,
    height: u32,

    bounds: Rect,

    texture: Option<Texture>,

    health_points: u32
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

            health_points: hp
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
        match self.texture {
            Some(ref tex) => renderer.copy(tex, Some(self.bounds), Some(Rect::new(self.x, self.y, self.width, self.height))),
            _ => ()
        }
    }

    pub fn process(&mut self, events: &mut Events, elapsed: f64, time: u32) {
    }

}
