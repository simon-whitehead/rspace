extern crate sdl2;
extern crate sdl2_image;

use sdl2::pixels::Color;
use sdl2::rect::Rect;

use engine::context::Context;
use engine::entities::FrameAnimatedSprite;
use engine::scene::{Scene, SceneResult};

use ::game::explosion::{Explosion, ExplosionResult};
use ::game::player::Player;

pub struct GameScene {
    bounds: Rect,
    player: Player,
    explosions: Vec<Explosion>
}

impl GameScene {
    pub fn new(width: u32, height: u32) -> GameScene {
        let bounds = Rect::new(0, 0, width, height);
        GameScene {
            bounds: bounds,
            player: Player::new(bounds),
            explosions: Vec::new()
        }
    }
}

impl Scene for GameScene {
    fn init(&mut self, context: &mut Context) {
        self.player.init(context);

        // Initialize 5 explosions for the screen
        let cache_result = context.texture_cache.precache(&context.renderer, "assets/explosion/large/");

        let bounds = self.get_bounds();

        for i in 0..5 {
            let mut sprite = FrameAnimatedSprite::new(0.1, bounds, cache_result.clone());
            sprite.init(context);

            self.explosions.push(Explosion::new((i * 60, i * 60), sprite));
        }
    }

    fn render(&mut self, context: &mut Context, elapsed: f64) -> SceneResult {
        let events = &mut context.event_handler;

        if events.quit || events.key_pressed(sdl2::keyboard::Keycode::Escape) {
            return SceneResult::Quit;
        }
        
        context.renderer.set_draw_color(Color::RGB(0, 0, 0));
        context.renderer.clear();

        self.player.render(&context.texture_cache, &mut context.renderer, elapsed);

        for explosion in &mut self.explosions {
            explosion.render(&context.texture_cache, &mut context.renderer, elapsed);
        }

        SceneResult::None
    }

    fn process(&mut self, context: &mut Context, elapsed: f64) -> SceneResult {
        for explosion in &mut self.explosions {
            explosion.process(elapsed);
        }

        // Keep only the explosions that haven't finished exploding
        self.explosions.retain(|explosion| !explosion.deleted);

        self.player.process(&mut context.event_handler, elapsed);
        
        SceneResult::None
    }

    fn get_bounds(&self) -> Rect {
        self.bounds
    }
}
