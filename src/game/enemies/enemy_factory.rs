
extern crate rand;
extern crate sdl2;

use game::enemies::enemy_factory::rand::{Rng, ThreadRng};

use sdl2::rect::Rect;

use ::engine::cache::AssetCacheResult;
use ::engine::context::Context;

use ::game::enemies::{Enemy, BasicEnemy};

pub struct EnemyFactory {
    positions: Vec<u32>,
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

    // Remove the last 5 enemy positions we have stored
    // so that we can use those locations again
    pub fn GC(&mut self) {
        if self.positions.len() > 10 {
            self.positions = self.positions.split_off(4);
        }
    }

    // This method attempts to not overlap enemies that have recently spawned.
    // It does this by referring back to a local cache of recent X positions
    // and generates a random number until none overlap.
    fn get_x(&mut self, threshold: u32) -> u32 {
        let mut random_x = self.rng.gen_range(0, self.bounds.width()-threshold);
        let mut found = false;
        let mut iterations = 0;

        if self.positions.len() == 0 {
            random_x
        } else {

            while !found {
                iterations += 1;
                // If we've tried for too long ... bail out and just let them overlap
                if iterations > 20 {
                    return random_x;
                }
                for p in &self.positions {
                    if random_x > *p {
                        if random_x > *p + threshold {
                            found = true;
                        }
                    } else {
                        if random_x < *p - threshold {
                            found = true;
                        }
                    }
                }

                random_x = self.rng.gen_range(0, self.bounds.width()-threshold);
            }

            random_x
        }
    }

    pub fn create_basic_enemy(&mut self, context: &mut Context, cache: AssetCacheResult) -> BasicEnemy {
        let mut enemy = BasicEnemy::new((0, 0), self.bounds, cache);
        enemy.init(context);

        let random_x = self.get_x(enemy.get_width());
        self.positions.push(random_x);

        let height = 0 - enemy.height as i32;
        enemy.set_x(random_x as i32);
        enemy.set_y(height as i32);


        enemy
    }
}
