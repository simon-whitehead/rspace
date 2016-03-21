extern crate rand;

extern crate sdl2;
extern crate sdl2_image;

use sdl2::pixels::Color;
use sdl2::rect::Rect;

use engine::cache::AssetCacheResult;
use engine::context::Context;
use engine::scene::{Scene, SceneResult};

use ::game::bullets::Bullet;
use ::game::debug::DebugPanel;
use ::game::enemies::{Enemy, EnemyFactory, EnemyAction, EnemyType};
use ::game::explosion::Explosion;
use ::game::levels::{LevelParser, Level, Level1, Level2, OpCode};
use ::game::player::{Player, PlayerProcessResult};

use ::game::scenes::GameOverScene;

pub struct GameScene {
    bounds: Rect,
    player: Player,

    levels: Vec<Box<Level>>,
    current_level: usize,
    level_parser: Option<LevelParser>,

    game_over_interval: u32,
    game_over_time: u32,

    explosions: Vec<Explosion>,
    bullets: Vec<Box<Bullet>>,
    enemies: Vec<Box<Enemy>>,
    enemy_factory: EnemyFactory,

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
            level_parser: None,

            game_over_interval: 2000,
            game_over_time: 0,

            explosions: Vec::new(),
            bullets: Vec::new(),
            enemies: Vec::new(),
            enemy_factory: EnemyFactory::new(bounds),

            large_explosion_cache: None,
            medium_explosion_cache: None,
            small_explosion_cache: None,
            tiny_explosion_cache: None,

            debug_panel: None
        }
    }

    fn process_level(&mut self, context: &mut Context) {
        let current_ticks = context.timer.ticks();

        if self.current_level >= self.levels.len() {
            // Start the "Game Over" sequence
            if self.game_over_time == 0 {
                self.game_over_time = current_ticks;
            }
            return;
        }

        if let Some(ref mut parser) = self.level_parser {
            match parser.process(&mut self.levels[self.current_level], current_ticks) {
                OpCode::SpawnEnemy(enemy_type) => {
                    match enemy_type {
                        EnemyType::BasicEnemy => {
                            // Spawn a BasicEnemy in the scene somewhere
                            if let Some(ref cache) = self.medium_explosion_cache {
                                let enemy = self.enemy_factory.create_basic_enemy(context, (*cache).clone());
                                self.enemies.push(Box::new(enemy));
                            }
                        }
                    }
                },
                OpCode::WaitFor(seconds) => {
                    parser.wait_for(seconds as u32);
                },
                OpCode::EndLevel => {
                    // No more enemies left?
                    if self.enemies.len() == 0 {
                        // Do we have more levels to play?
                        if self.current_level < self.levels.len() {
                            // Switch levels
                            parser.reset();
                            parser.wait_for(5);
                            self.current_level += 1;

                            return;
                        }
                    } else {
                        // Wait at least another second before processing another opcode (to give
                        // the user time to finish the level)
                        parser.wait_for(1);
                    }
                },
                OpCode:: None => {
                    ()
                }
            }
        }
    }

    // Process each enemy
    fn process_enemies(&mut self, context: &mut Context, elapsed: f64) {
        // Clean up the enemy factory
        self.enemy_factory.gc();

        for enemy in &mut self.enemies {
            match enemy.process(&mut context.event_handler, elapsed, context.timer.ticks()) {
                EnemyAction::Shoot => {
                    self.bullets.append(&mut enemy.shoot());
                },
                _ => ()
            }

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
        // For every bullet we have in the scene... process it
        for bullet in &mut self.bullets {
            bullet.process(context);

            // If the player owns this bullet
            if bullet.is_player_owned() {
                // Check if the bullet hit an enemy
                for enemy in &mut self.enemies {
                    if enemy.hit_test(Rect::new(bullet.get_x(), bullet.get_y(), 2, 6)) {
                        // If it did ... delete this bullet
                        bullet.delete();

                        // Tell the enemy it was damaged
                        enemy.take_damage(bullet.damage());
                    }
                }
            } else {
                // Otherwise an enemy owns the bullet... check if the bullet hit the player
                if self.player.hit_test(Rect::new(bullet.get_x(), bullet.get_y(), 2, 6)) {
                    bullet.delete();
                    self.player.take_damage(bullet.damage());
                }
            }
        }

        // Clear out our bullets and only keep the ones that aren't deleted
        let old_bullets = ::std::mem::replace(&mut self.bullets, vec![]);
        self.bullets = old_bullets.into_iter().filter(|bullet| !bullet.is_deleted()).collect();
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

        // Initialize explosion cached assets
        let large_explosion_cache = context.texture_cache.precache(&context.renderer, "assets/explosion/large/");
        let medium_explosion_cache = context.texture_cache.precache(&context.renderer, "assets/explosion/medium/");
        let small_explosion_cache = context.texture_cache.precache(&context.renderer, "assets/explosion/small/");
        let tiny_explosion_cache = context.texture_cache.precache(&context.renderer, "assets/explosion/tiny/");

        self.player.init(context, large_explosion_cache.clone());

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

        self.level_parser = Some(LevelParser::new());
    }

    fn render(&mut self, context: &mut Context, elapsed: f64) -> SceneResult {
        let events = &mut context.event_handler;

        // Quit if someone quits or Game Over is done
        if events.quit || events.key_pressed(sdl2::keyboard::Keycode::Escape) {
            return SceneResult::Quit;
        }
        
        context.renderer.set_draw_color(Color::RGB(0, 0, 0));
        context.renderer.clear();

        if self.player.alive() {
            self.player.render(&mut context.renderer);
        }

        for enemy in &mut self.enemies {
            enemy.render(&context.texture_cache, &mut context.renderer, elapsed);
        }

        for explosion in &mut self.explosions {
            explosion.render(&context.texture_cache, &mut context.renderer);
        }

        for bullet in &mut self.bullets {
            bullet.render(&mut context.renderer);
        }

        if context.DEBUG {
            if let Some(ref mut debug_panel) = self.debug_panel {
                debug_panel.render(&mut context.renderer);
            }
        }

        SceneResult::None
    }

    fn process(&mut self, context: &mut Context, elapsed: f64) -> SceneResult {
        if self.game_over_time > 0 && (context.timer.ticks() - self.game_over_time >= self.game_over_interval) {
            // Game over ... lets change the scene
            return SceneResult::ChangeScene(Box::new(GameOverScene::new(self.get_bounds())));
        }

        // Handle player actions if they are still alive
        if self.game_over_time == 0 {
            match self.player.process(&mut self.enemies, &mut context.event_handler, context.timer.ticks()) {
                PlayerProcessResult::Shoot => self.bullets.append(&mut self.player.shoot()),
                PlayerProcessResult::Dead => {
                    self.explosions.append(&mut self.player.explode(context));
                    self.game_over_time = context.timer.ticks();
                }
                _ => ()
            }
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
                debug_panel.set_player_health(self.player.health_points);
                debug_panel.set_active_explosions(self.explosions.len() as u32);
                debug_panel.set_enemies(self.enemies.len() as u32);
                debug_panel.set_bullets(self.bullets.len() as u32);

                if let Some(ref level_parser) = self.level_parser {
                    debug_panel.set_level_info(self.current_level as u32, level_parser.current_opcode());
                }
            }
        }

        SceneResult::None
    }

    fn get_bounds(&self) -> Rect {
        self.bounds
    }
}
