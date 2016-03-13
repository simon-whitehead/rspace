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
use ::game::enemies::BasicEnemy;
use ::game::explosion::{Explosion, ExplosionResult};
use ::game::player::{Player, PlayerProcessResult};

pub struct GameScene {
    bounds: Rect,
    player: Player,
    explosions: Vec<Explosion>,
    explosion_interval: u32,
    last_explosion_interval: u32,
    explosion_counter: Option<::engine::text::Text>,

    bullets: Vec<Bullet>,

    basic_enemies: Vec<BasicEnemy>,

    cache: Option<AssetCacheResult>
}

impl GameScene {
    pub fn new(width: u32, height: u32) -> GameScene {
        let bounds = Rect::new(0, 0, width, height);
        GameScene {
            bounds: bounds,
            player: Player::new(bounds),
            explosions: Vec::new(),
            explosion_interval: 1_000,
            last_explosion_interval: 0,
            explosion_counter: None,

            bullets: Vec::new(),

            basic_enemies: Vec::new(),

            cache: None
        }
    }
}

impl Scene for GameScene {
    fn init(&mut self, context: &mut Context) {
        let mut enemy = BasicEnemy::new((350, 50), 100, context.bounds);
        enemy.init(context);
        self.basic_enemies.push(enemy);

        self.player.init(context);

        let mut explosion_counter = Text::new((200, 10), "Active explosions: 0", 24, Color::RGBA(255, 255, 0, 255), "assets/fonts/OpenSans-Bold.ttf", self.get_bounds());
                
        explosion_counter.init(context);
        self.explosion_counter = Some(explosion_counter);

        // Initialize 5 explosions for the screen
        self.cache = Some(context.texture_cache.precache(&context.renderer, "assets/explosion/large/"));
    }

    fn render(&mut self, context: &mut Context, elapsed: f64) -> SceneResult {
        let events = &mut context.event_handler;

        if events.quit || events.key_pressed(sdl2::keyboard::Keycode::Escape) {
            return SceneResult::Quit;
        }
        
        context.renderer.set_draw_color(Color::RGB(0, 0, 0));
        context.renderer.clear();

        self.player.render(&context.texture_cache, &mut context.renderer, elapsed);

        for enemy in &mut self.basic_enemies {
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

        for explosion in &mut self.explosions {
            explosion.process(elapsed);
        }

        for bullet in &mut self.bullets {
            bullet.process();

            // Spawn an explosion at the edge of the screen where the bullet died
            if bullet.deleted {
                if let Some(ref cache) = self.cache {
                    let mut sprite = FrameAnimatedSprite::new(0.05, bounds, (*cache).clone());
                    sprite.init(context);

                    let x = bullet.x - sprite.width as i32 / 2;
                    let y = bullet.y - (sprite.height as i32 / 2) + sprite.height as i32 / 8;
                    self.explosions.push(Explosion::new((x, y), sprite));
                }
            }
        }

        // Keep only the explosions that haven't finished exploding
        self.explosions.retain(|explosion| !explosion.deleted);

        // Keep only the bullets still on the screen
        self.bullets.retain(|bullet| !bullet.deleted);

        if let Some(ref mut explosion_counter) = self.explosion_counter {
            explosion_counter.set_text(format!("Active explosions: {}", self.explosions.len()));
        }

        match self.player.process(&mut context.event_handler, elapsed, context.timer.ticks()) {
            PlayerProcessResult::Shoot => self.bullets.append(&mut self.player.shoot()),
            _ => ()
        }
        
        SceneResult::None
    }

    fn get_bounds(&self) -> Rect {
        self.bounds
    }
}
