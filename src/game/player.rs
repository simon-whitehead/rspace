extern crate sdl2;

use sdl2::pixels::Color;

use ::engine::context::Context;
use ::engine::entity::Entity;
use ::engine::scene::{SceneResult};

pub struct Player {
    top: i32,
    left: i32
}

impl Player {
    pub fn new() -> Player {
        Player {
            top: 0i32,
            left: 0i32
        }
    }
}

impl Entity for Player {
    fn render(&mut self, renderer: &mut sdl2::render::Renderer, elapsed: f64) {
        renderer.set_draw_color(Color::RGB(255, 255, 255));

        renderer.fill_rect(sdl2::rect::Rect::new(self.left, self.top, 50, 50));
    }

    fn process(&mut self, events: &mut ::engine::events::Events, elapsed: f64) {
        if events.key_pressed(::sdl2::keyboard::Keycode::Up) {
            self.top -= 10;
        }

        if events.key_pressed(::sdl2::keyboard::Keycode::Down) {
            self.top += 10;
        }

        if events.key_pressed(::sdl2::keyboard::Keycode::Left) {
            self.left -= 10;
        }

        if events.key_pressed(::sdl2::keyboard::Keycode::Right) {
            self.left += 10;
        }

    }
}
