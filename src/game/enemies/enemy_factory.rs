
extern crate rand;
extern crate sdl2;

use game::enemies::enemy_factory::rand::{Rng, ThreadRng};

use sdl2::rect::{Point, Rect};

use ::engine::cache::AssetCacheResult;
use ::engine::context::Context;

use ::game::enemies::{Enemy, BasicEnemy};

pub struct EnemyFactory {
    positions: Vec<Point>,
    bounds: Rect,
    rng: rand::ThreadRng
}

impl EnemyFactory {
    pub fn new(bounds: Rect) -> EnemyFactory {
       EnemyFactory {
           positions: Vec::new(),
           bounds: bounds,
           rng: rand::thread_rng()
       }
    }

    pub fn create_basic_enemy(&mut self, context: &mut Context, cache: AssetCacheResult) -> BasicEnemy {
        let random_x = self.rng.gen_range(0, self.bounds.width()) as i32;

        let mut enemy = BasicEnemy::new((random_x, 0), self.bounds, cache);
        enemy.init(context);
        let height = 0 - enemy.height as i32;
        enemy.set_y(height as i32);

        enemy
    }
}
