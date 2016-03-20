
extern crate rand;

extern crate sdl2;
extern crate sdl2_image;

use game::enemies::basic_enemy::rand::{Rng, ThreadRng};

use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, TextureQuery, Renderer};

use sdl2_image::LoadTexture;

use std::path::Path;

use ::engine::cache::{AssetCacheResult, TextureCache};
use ::engine::context::Context;
use ::engine::entities::FrameAnimatedSprite;
use ::engine::events::Events;

use ::game::bullets::{Bullet, BasicEnemyBullet};
use ::game::enemies::{EnemyAction, Enemy};
use ::game::explosion::Explosion;

static HEALTH: i32 = 100;

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
    last_move_time: u32,

    shoot_interval: u32,
    last_shoot_time: u32,

    rng: ThreadRng
}

impl BasicEnemy {
    pub fn new(position: (i32, i32),
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

            health_points: HEALTH,
            dead: false,

            move_interval: 50,   // Every 50 milliseconds, move down the screen slightly
            last_move_time: 0,

            shoot_interval: 2000,   // Dynamic... between 2 and 5 seconds though
            last_shoot_time: 0,

            rng: rand::thread_rng()
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

    fn render(&mut self, _asset_cache: &TextureCache, renderer: &mut Renderer, _elapsed: f64) {
        // Render the ship
        if let Some(ref tex) = self.texture {
            renderer.copy_ex(tex, Some(self.bounds), Some(Rect::new(self.x, self.y, self.width, self.height)), 0.0, Some(Point::new(self.width as i32 / 2, self.height as i32 / 2)), false, true).unwrap();
        }

        // Render the health bar
        renderer.set_draw_color(Color::RGB(255, 255, 0));
        let health_percentage: f64 = self.health_points as f64 / HEALTH as f64;
        let bar_x = self.x;
        let bar_y = self.y + 10;

        renderer.fill_rect(Rect::new(bar_x, bar_y, (self.width as f64 * health_percentage) as u32, 5)).unwrap();
    }

    fn process(&mut self, _events: &mut Events, _elapsed: f64, time: u32) -> EnemyAction {
        let mut result = EnemyAction::None;

        // Should we move down slightly?
        if time - self.last_move_time >= self.move_interval {
            self.y += 1;

            self.last_move_time = time;
        }

        if time - self.last_shoot_time >= self.shoot_interval {
            result = EnemyAction::Shoot;

            self.last_shoot_time = time;

            // Randomly choose a time between the next 2 and 6 seconds to shoot again
            self.shoot_interval = self.rng.gen_range(2, 5) * 1000;
        }

        result
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

    fn shoot(&self) -> Vec<Box<Bullet>> {
        vec![

            Box::new(BasicEnemyBullet::new((self.x + self.width as i32 / 2, self.y + self.height as i32)))

        ]
    }
}
