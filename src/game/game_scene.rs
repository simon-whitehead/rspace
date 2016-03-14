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
use ::game::enemies::{Enemy, BasicEnemy};
use ::game::explosion::{Explosion, ExplosionResult};
use ::game::player::{Player, PlayerProcessResult};

pub struct GameScene {
    bounds: Rect,
    player: Player,
    explosions: Vec<Explosion>,
    explosion_counter: Option<::engine::text::Text>,

    bullets: Vec<Bullet>,

    enemies: Vec<Box<Enemy>>,

    large_explosion_cache: Option<AssetCacheResult>,
    medium_explosion_cache: Option<AssetCacheResult>,
    small_explosion_cache: Option<AssetCacheResult>,
    tiny_explosion_cache: Option<AssetCacheResult>
}

impl GameScene {
    pub fn new(width: u32, height: u32) -> GameScene {
        let bounds = Rect::new(0, 0, width, height);
        GameScene {
            bounds: bounds,
            player: Player::new(bounds),
            explosions: Vec::new(),
            explosion_counter: None,

            bullets: Vec::new(),

            enemies: Vec::new(),

            large_explosion_cache: None,
            medium_explosion_cache: None,
            small_explosion_cache: None,
            tiny_explosion_cache: None
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
        self.player.init(context);

        let mut explosion_counter = Text::new((200, 10), "Active explosions: 0", 24, Color::RGBA(255, 255, 0, 255), "assets/fonts/OpenSans-Bold.ttf", self.get_bounds());
                
        explosion_counter.init(context);
        self.explosion_counter = Some(explosion_counter);

        // Initialize explosion cached assets
        let large_explosion_cache = context.texture_cache.precache(&context.renderer, "assets/explosion/large/");
        let medium_explosion_cache = context.texture_cache.precache(&context.renderer, "assets/explosion/medium/");
        let small_explosion_cache = context.texture_cache.precache(&context.renderer, "assets/explosion/small/");
        let tiny_explosion_cache = context.texture_cache.precache(&context.renderer, "assets/explosion/tiny/");

        // Create a basic enemy that uses the Medium explosion
        let basic_explosion = medium_explosion_cache.clone();
        let mut enemy = BasicEnemy::new((350, 50), 100, context.bounds, basic_explosion);
        enemy.init(context);
        self.enemies.push(Box::new(enemy));

        // Store our caches for later
        self.large_explosion_cache = Some(large_explosion_cache);
        self.medium_explosion_cache = Some(medium_explosion_cache);
        self.small_explosion_cache = Some(small_explosion_cache);
        self.tiny_explosion_cache = Some(tiny_explosion_cache);
    }

    fn render(&mut self, context: &mut Context, elapsed: f64) -> SceneResult {
        let events = &mut context.event_handler;

        if events.quit || events.key_pressed(sdl2::keyboard::Keycode::Escape) {
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

        if let Some(ref mut explosion_counter) = self.explosion_counter {
            explosion_counter.render(&mut context.renderer, elapsed);
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

        // Handle enemies
        self.process_enemies(context, elapsed);

        // Handle explosions
        self.process_explosions(elapsed);

        // Handle the bullets
        self.process_bullets(context);

        if let Some(ref mut explosion_counter) = self.explosion_counter {
            explosion_counter.set_text(format!("Active explosions: {}", self.explosions.len()));
        }
        
        SceneResult::None
    }

    fn get_bounds(&self) -> Rect {
        self.bounds
    }
}
