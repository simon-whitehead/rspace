extern crate sdl2;
extern crate sdl2_image;

use std::path::Path;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Renderer, Texture, TextureQuery};

use sdl2_image::LoadTexture;

use ::engine::cache::TextureCache;
use ::engine::context::Context;
use ::engine::events::Events;
use ::engine::scene::{Scene, SceneResult};
use ::engine::window::Window;

use game::bullet::Bullet;

pub enum PlayerProcessResult {
    None,
    Shoot
}

pub struct Player {
    top: i32,
    left: i32,

    width: u32,
    height: u32,
    
    bounds: Rect,
    texture: Option<Texture>,

    shoot_interval: u32,
    last_shoot_time: u32
}

impl Player {
    pub fn new(bounds: Rect) -> Player {
        Player {
            top: 0i32,
            left: 0i32,

            width: 0,
            height: 0,

            bounds: bounds,
            texture: None,
            
            shoot_interval: 100,
            last_shoot_time: 0
        }
    }

    pub fn init(&mut self, context: &mut Context) {
        let tex = context.renderer.load_texture(Path::new("assets/player/ship.png")).unwrap();

        let TextureQuery { width, height, .. } = tex.query();

        self.width = width >> 2;
        self.height = height >> 2;

        self.texture = Some(tex);
    }

    pub fn render(&mut self, asset_cache: &TextureCache, renderer: &mut Renderer, elapsed: f64) {
        match self.texture {
            Some(ref tex) => renderer.copy(tex, Some(self.bounds), Some(Rect::new(self.left, self.top, self.width, self.height))),
            _ => ()
        }
    }

    pub fn process(&mut self, events: &mut Events, elapsed: f64, time: u32) -> PlayerProcessResult {
        if events.key_pressed(::sdl2::keyboard::Keycode::Up) {
            self.top = ::engine::helpers::clamp_min(self.top, -10, 0);
        }

        if events.key_pressed(::sdl2::keyboard::Keycode::Down) {
            self.top = ::engine::helpers::clamp_max(self.top, 10, self.bounds.bottom() as i32 - self.height as i32);
        }

        if events.key_pressed(::sdl2::keyboard::Keycode::Left) {
            self.left = ::engine::helpers::clamp_min(self.left, -10, 0);
        }

        if events.key_pressed(::sdl2::keyboard::Keycode::Right) {
            self.left = ::engine::helpers::clamp_max(self.left, 10, self.bounds.right() as i32 - self.width as i32);
        }

        if events.key_pressed(::sdl2::keyboard::Keycode::Space) {
            if time - self.last_shoot_time > self.shoot_interval {
                self.last_shoot_time = time;
                return PlayerProcessResult::Shoot 
            }
        }

        PlayerProcessResult::None
    }

    pub fn shoot(&self) -> Vec<Bullet> {
        vec![

            Bullet::new((self.left + self.width as i32 / 2, self.top))

        ]
    }
}
