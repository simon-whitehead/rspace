extern crate sdl2;
extern crate sdl2_image;

use std::path::Path;

use sdl2::rect::Rect;
use sdl2::render::{Renderer, Texture, TextureQuery};

use sdl2_image::LoadTexture;

use ::engine::cache::AssetCacheResult;
use ::engine::context::Context;
use ::engine::entities::FrameAnimatedSprite;
use ::engine::events::Events;

use game::bullets::{Bullet, MachineGunBullet};
use game::enemies::Enemy;
use game::explosion::Explosion;

pub enum PlayerProcessResult {
    None,
    Shoot,
    Dead
}

pub struct Player {
    pub health_points: i32,

    y: i32,
    x: i32,

    width: u32,
    height: u32,
    
    bounds: Rect,
    texture: Option<Texture>,

    // Weapons
    machinegun: Option<AssetCacheResult>,

    explosion_cache: Option<AssetCacheResult>,

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

            // Weapons
            machinegun: None,

            explosion_cache: None,

            health_points: 100,
            
            shoot_interval: 100,
            last_shoot_time: 0
        }
    }

    pub fn init(&mut self, context: &mut Context, explosion_cache: AssetCacheResult, machinegun_cache: AssetCacheResult) {
        let tex = context.renderer.load_texture(Path::new("assets/player/ship.png")).unwrap();

        let TextureQuery { width, height, .. } = tex.query();

        self.width = width >> 2;
        self.height = height >> 2;

        self.x = (self.bounds.width() as i32 / 2) - self.width as i32 / 2;
        self.y = self.bounds.height() as i32 - self.height as i32;

        self.texture = Some(tex);
        self.machinegun = Some(machinegun_cache);
        self.explosion_cache = Some(explosion_cache);
    }

    pub fn render(&mut self, renderer: &mut Renderer) {
        match self.texture {
            Some(ref tex) => renderer.copy(tex, Some(self.bounds), Some(Rect::new(self.x, self.y, self.width, self.height))),
            _ => ()
        }
    }

    pub fn process(&mut self, enemies: &mut Vec<Box<Enemy>>, events: &mut Events, time: u32) -> PlayerProcessResult {
        // Handle key presses
        let mut result = self.process_keys(events, time);

        // Have we collided with an enemy?
        for enemy in enemies {
            let player_rect = Rect::new(self.x, self.y, self.width, self.height);
            let enemy_rect = Rect::new(enemy.get_x(), enemy.get_y(), enemy.get_width(), enemy.get_height());

            if ::engine::helpers::overlap(player_rect, enemy_rect) {
                // We collided ... kill the enemy
                enemy.take_damage(999999);
                // Take some damage ourselves
                self.take_damage(50);
            }
        }

        if self.health_points <= 0 {
            result = PlayerProcessResult::Dead;
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

    pub fn shoot(&self) -> Vec<Box<Bullet>> {
        if let Some(ref machinegun) = self.machinegun {
            let cache = (*machinegun).clone();
            // Half the width of the player - half the width of the bullet
            let x_pos = (self.width / 2) - (cache.width / 2);
            vec![

                Box::new(MachineGunBullet::new((self.x + x_pos as i32, self.y), (cache.width, cache.height), cache, self.bounds))

            ]
        } else {
            vec![]
        }
    }

    pub fn hit_test(&mut self, rect: sdl2::rect::Rect) -> bool {
        let player_rect = Rect::new(self.x, self.y, self.width, self.height);

        ::engine::helpers::overlap(player_rect, rect)
    }

    pub fn take_damage(&mut self, damage: i32) {
        self.health_points -= damage;
    }

    // Generate explosions for the player
    pub fn explode(&self, context: &mut Context) -> Vec<Explosion> {
        let mut result = Vec::new();
        let bounds = self.bounds;
        let start_x = self.x + (self.width as i32 / 2);
        let start_y = self.y + (self.height as i32 / 2);

        if let Some(ref cache) = self.explosion_cache {
            let mut sprite = FrameAnimatedSprite::new(0.05, bounds, (*cache).clone());
            sprite.init(context);

            let x = start_x - sprite.width as i32 / 2;
            let y = start_y - (sprite.height as i32 / 2);
            result.push(Explosion::new((x, y), sprite));
        }

        result
    }

    pub fn alive(&self) -> bool {
        self.health_points > 0
    }
}
