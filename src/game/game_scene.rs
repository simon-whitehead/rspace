extern crate rand;

extern crate sdl2;
extern crate sdl2_image;

use game::game_scene::rand::Rng;

use sdl2::pixels::Color;
use sdl2::rect::Rect;

use engine::cache::AssetCacheResult;
use engine::context::Context;
use engine::entities::FrameAnimatedSprite;
use engine::scene::{Scene, SceneResult};
use engine::text::Text;

use ::game::bullet::Bullet;
use ::game::debug::DebugPanel;
use ::game::enemies::{Enemy, BasicEnemy, EnemyType};
use ::game::explosion::{Explosion, ExplosionResult};
use ::game::levels::{Level, Level1, Level2, OpCode};
use ::game::player::{Player, PlayerProcessResult};

pub struct GameScene {
    bounds: Rect,
    player: Player,

    levels: Vec<Box<Level>>,
    current_level: usize,
    level_opcode: usize,
    opcode_wait: u32,
    last_opcode_time: u32,

    game_over_interval: u32,
    game_over_time: u32,

    explosions: Vec<Explosion>,
    explosion_counter: Option<::engine::text::Text>,

    bullets: Vec<Bullet>,

    enemies: Vec<Box<Enemy>>,

    large_explosion_cache: Option<AssetCacheResult>,
    medium_explosion_cache: Option<AssetCacheResult>,
    small_explosion_cache: Option<AssetCacheResult>,
    tiny_explosion_cache: Option<AssetCacheResult>,

    debug_panel: Option<DebugPanel>
}

impl GameScene {
    pub fn new(width: u32, height: u32) -> GameScene {
        let bounds = Rect::new(0, 0, width, height);
        GameScene {
            bounds: bounds,
            player: Player::new(bounds),

            levels: Vec::new(),
            current_level: 0,
            level_opcode: 0,
            opcode_wait: 0,
            last_opcode_time: 0,

            game_over_interval: 2000,
            game_over_time: 0,

            explosions: Vec::new(),
            explosion_counter: None,

            bullets: Vec::new(),

            enemies: Vec::new(),

            large_explosion_cache: None,
            medium_explosion_cache: None,
            small_explosion_cache: None,
            tiny_explosion_cache: None,

            debug_panel: None
        }
    }

    fn process_level(&mut self, context: &mut Context) {
        let mut rng = rand::thread_rng();
        let current_ticks = context.timer.ticks();

        if current_ticks - self.last_opcode_time >= self.opcode_wait {
            // We should process another level opcode now

            let level_count = self.levels.len();
            let mut current_level = &mut self.levels[self.current_level];
            let next_opcode = current_level.get(self.level_opcode);

            match next_opcode {
                OpCode::SpawnEnemy(enemy_type) => {
                    match enemy_type {
                        EnemyType::BasicEnemy => {
                            if let Some(ref cache) = self.medium_explosion_cache {
                                let random_x = rng.gen_range(0, context.bounds.width()) as i32;

                                let mut enemy = BasicEnemy::new((random_x, 0), context.bounds, (*cache).clone());
                                enemy.init(context);
                                let height = 0 - enemy.height as i32;
                                enemy.set_y(height as i32);
                                self.enemies.push(Box::new(enemy));
                            }
                        }
                    }
                },
                OpCode::WaitFor(seconds) => {
                    self.opcode_wait = context.timer.ticks() + seconds as u32 * 1000;
                },
                OpCode::EndLevel => {
                    // No more enemies left?
                    if self.enemies.len() == 0 {
                        // Do we have more levels to play?
                        if self.current_level == level_count - 1 {
                            // Nope.. start the "Game Over" sequence
                            self.game_over_time = current_ticks;
                            self.opcode_wait = 999 * 1000;
                        } else {
                            // Switch levels
                            self.opcode_wait = current_ticks + 5 as u32 * 1000; // Wait 5 seconds before starting the next level
                            self.current_level += 1;
                            self.level_opcode = 0;
                        }
                    } else {
                        // Wait at least another second before processing another opcode (to give
                        // the user time to finish the level)
                        self.opcode_wait = current_ticks + 1000;
                    }
                },
                OpCode:: None => {
                    ()
                }
            }

            self.level_opcode += 1;
        }
    }

    // Process each enemy
    fn process_enemies(&mut self, context: &mut Context, elapsed: f64) {
        for enemy in &mut self.enemies {
            enemy.process(&mut context.event_handler, elapsed, context.timer.ticks());

            // Have the enemy explode and die
            if enemy.is_dead() {
                let mut explosions = &mut enemy.explode(context);
                self.explosions.append(explosions);
            }
        }

        // Clear out our enemies and only keep the ones that aren't dead
        let old_enemies = ::std::mem::replace(&mut self.enemies, vec![]);
        self.enemies = old_enemies.into_iter().filter(|enemy| !enemy.is_dead()).collect();
    }

    // Process each explosion and remove any deleted ones
    fn process_explosions(&mut self, elapsed: f64) {
        for explosion in &mut self.explosions {
            explosion.process(elapsed);
        }

        // Clear out our explosions and only keep the ones that aren't deleted
        let old_explosions = ::std::mem::replace(&mut self.explosions, vec![]);
        self.explosions = old_explosions.into_iter().filter(|explosion| !explosion.deleted).collect();
    }

    fn process_bullets(&mut self, context: &mut Context) {
        let bounds = self.get_bounds();

        // For every bullet we have in the scene... process it
        for bullet in &mut self.bullets {
            bullet.process();

            // Check if the bullet hit an enemy
            for enemy in &mut self.enemies {
                if enemy.hit_test(Rect::new(bullet.x, bullet.y, 2, 6)) {
                    // If it did ... delete this bullet
                    bullet.deleted = true;

                    // Tell the enemy it was damaged
                    enemy.take_damage(10);
                }
            }
        }

        // Clear out our bullets and only keep the ones that aren't deleted
        let old_bullets = ::std::mem::replace(&mut self.bullets, vec![]);
        self.bullets = old_bullets.into_iter().filter(|bullet| !bullet.deleted).collect();
    }
}

impl Scene for GameScene {
    fn init(&mut self, context: &mut Context) {
        let bounds = self.get_bounds();

        if context.DEBUG {
            let mut panel = DebugPanel::new();
            panel.init(context, bounds);

            self.debug_panel = Some(panel);
        }

        self.player.init(context);

        // Initialize explosion cached assets
        let large_explosion_cache = context.texture_cache.precache(&context.renderer, "assets/explosion/large/");
        let medium_explosion_cache = context.texture_cache.precache(&context.renderer, "assets/explosion/medium/");
        let small_explosion_cache = context.texture_cache.precache(&context.renderer, "assets/explosion/small/");
        let tiny_explosion_cache = context.texture_cache.precache(&context.renderer, "assets/explosion/tiny/");

        // Store our caches for later
        self.large_explosion_cache = Some(large_explosion_cache);
        self.medium_explosion_cache = Some(medium_explosion_cache);
        self.small_explosion_cache = Some(small_explosion_cache);
        self.tiny_explosion_cache = Some(tiny_explosion_cache);

        self.levels = vec![

            Box::new(Level1::new()),
            Box::new(Level2::new())

        ];

        self.levels[self.current_level].init();
    }

    fn render(&mut self, context: &mut Context, elapsed: f64) -> SceneResult {
        let events = &mut context.event_handler;

        // Quit if someone quits or Game Over is done
        if (events.quit || events.key_pressed(sdl2::keyboard::Keycode::Escape)) ||
            self.game_over_time > 0 && (context.timer.ticks() - self.game_over_time >= self.game_over_interval) {
            return SceneResult::Quit;
        }
        
        context.renderer.set_draw_color(Color::RGB(0, 0, 0));
        context.renderer.clear();

        self.player.render(&context.texture_cache, &mut context.renderer, elapsed);

        for enemy in &mut self.enemies {
            enemy.render(&context.texture_cache, &mut context.renderer, elapsed);
        }

        for explosion in &mut self.explosions {
            explosion.render(&context.texture_cache, &mut context.renderer, elapsed);
        }

        for bullet in &mut self.bullets {
            bullet.render(&mut context.renderer);
        }

        if context.DEBUG {
            if let Some(ref mut debug_panel) = self.debug_panel {
                debug_panel.render(&mut context.renderer, elapsed);
            }
        }

        SceneResult::None
    }

    fn process(&mut self, context: &mut Context, elapsed: f64) -> SceneResult {

        let bounds = self.get_bounds();

        // Handle player actions
        match self.player.process(&mut self.enemies, &mut context.event_handler, elapsed, context.timer.ticks()) {
            PlayerProcessResult::Shoot => self.bullets.append(&mut self.player.shoot()),
            _ => ()
        }

        // Handle the level VM
        self.process_level(context);

        // Handle enemies
        self.process_enemies(context, elapsed);

        // Handle explosions
        self.process_explosions(elapsed);

        // Handle the bullets
        self.process_bullets(context);

        if context.DEBUG {
            if let Some(ref mut debug_panel) = self.debug_panel {
                debug_panel.set_active_explosions(self.explosions.len() as u32);
                debug_panel.set_enemies(self.enemies.len() as u32);
                debug_panel.set_bullets(self.bullets.len() as u32);

                debug_panel.set_level_info(self.current_level as u32, self.levels[self.current_level].get(self.level_opcode));
            }
        }

        SceneResult::None
    }

    fn get_bounds(&self) -> Rect {
        self.bounds
    }
}
