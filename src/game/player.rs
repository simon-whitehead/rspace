extern crate sdl2;
extern crate sdl2_image;

use std::path::Path;

use sdl2::pixels::Color;
use sdl2::render::{Texture, TextureQuery};

use sdl2_image::LoadTexture;

use ::engine::context::Context;
use ::engine::entity::Entity;
use ::engine::scene::{Scene, SceneResult};
use ::engine::window::Window;

pub struct Player {
    top: i32,
    left: i32,

    width: u32,
    height: u32,
    
    bounds: sdl2::rect::Rect,
    texture: Option<sdl2::render::Texture>
}

impl Player {
    pub fn new() -> Player {
        Player {
            top: 0i32,
            left: 0i32,

            width: 251,
            height: 280,

            bounds: sdl2::rect::Rect::new(0, 0, 0, 0),
            texture: None
        }
    }

    pub fn set_bounds(&mut self, rect: sdl2::rect::Rect) {
        self.bounds = rect;
    }
}

impl Entity for Player {
    fn init(&mut self, renderer: &mut sdl2::render::Renderer) {
        self.texture = Some(renderer.load_texture(Path::new("assets/player/ship.png")).unwrap());
    }

    fn render(&mut self, renderer: &mut sdl2::render::Renderer, elapsed: f64) {
        renderer.set_draw_color(Color::RGB(255, 255, 255));

        renderer.fill_rect(sdl2::rect::Rect::new(self.left, self.top, self.width, self.height));
    }

    fn process(&mut self, events: &mut ::engine::events::Events, elapsed: f64) {
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
    }
}
