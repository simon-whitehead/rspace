extern crate sdl2;
extern crate sdl2_image;

use std::path::Path;

use sdl2::rect::Rect;
use sdl2::render::{Renderer, Texture, TextureQuery};

use sdl2_image::LoadTexture;

use ::engine::context::Context;
use ::engine::events::Events;

use game::bullet::Bullet;
use game::enemies::Enemy;

pub enum PlayerProcessResult {
    None,
    Shoot
}

pub struct Player {
    y: i32,
    x: i32,

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
            x: 0i32,
            y: 0i32,

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

        self.x = (self.bounds.width() as i32 / 2) - self.width as i32 / 2;
        self.y = self.bounds.height() as i32 - self.height as i32;

        self.texture = Some(tex);
    }

    pub fn render(&mut self, renderer: &mut Renderer) {
        match self.texture {
            Some(ref tex) => renderer.copy(tex, Some(self.bounds), Some(Rect::new(self.x, self.y, self.width, self.height))),
            _ => ()
        }
    }

    pub fn process(&mut self, enemies: &mut Vec<Box<Enemy>>, events: &mut Events, time: u32) -> PlayerProcessResult {
        // Handle key presses
        let result = self.process_keys(events, time);

        // Have we collided with an enemy?
        for enemy in enemies {
            let player_rect = Rect::new(self.x, self.y, self.width, self.height);
            let enemy_rect = Rect::new(enemy.get_x(), enemy.get_y(), enemy.get_width(), enemy.get_height());

            if ::engine::helpers::overlap(player_rect, enemy_rect) {
                // We collided ... kill the enemy
                enemy.take_damage(999999);
            }
        }

        result
    }

    fn process_keys(&mut self, events: &mut Events, time: u32) -> PlayerProcessResult {
        if events.key_pressed(::sdl2::keyboard::Keycode::Up) {
            self.y = ::engine::helpers::clamp_min(self.y, -10, 0);
        }

        if events.key_pressed(::sdl2::keyboard::Keycode::Down) {
            self.y = ::engine::helpers::clamp_max(self.y, 10, self.bounds.bottom() as i32 - self.height as i32);
        }

        if events.key_pressed(::sdl2::keyboard::Keycode::Left) {
            self.x = ::engine::helpers::clamp_min(self.x, -10, 0);
        }

        if events.key_pressed(::sdl2::keyboard::Keycode::Right) {
            self.x = ::engine::helpers::clamp_max(self.x, 10, self.bounds.right() as i32 - self.width as i32);
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

            Bullet::new((self.x + self.width as i32 / 2, self.y))

        ]
    }
}
