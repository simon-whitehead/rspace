
extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Rect;

use ::engine::context::Context;
use ::engine::scene::{Scene, SceneResult};
use ::engine::text::Text;

pub struct GameOverScene {
    bounds: Rect,
    game_over_text: Option<Text>
}

impl GameOverScene {
    pub fn new(bounds: Rect) -> GameOverScene {
        GameOverScene {
            bounds: bounds,
            game_over_text: None
        }
    }
}

impl Scene for GameOverScene {
    fn init(&mut self, context: &mut Context) {
        let mut game_over_text = Text::new((100, self.bounds.height() as i32 / 2), "GAME OVER", 24, Color::RGBA(255, 255, 0, 255), "assets/fonts/OpenSans-Bold.ttf", self.bounds);
        game_over_text.init(context);
        self.game_over_text = Some(game_over_text);
    }

    fn render(&mut self, context: &mut Context, _elapsed: f64) -> SceneResult {
        context.renderer.set_draw_color(Color::RGB(0, 0, 0));
        context.renderer.clear();

        context.renderer.set_draw_color(Color::RGB(255, 255, 0));

        if let Some(ref mut game_over_text) = self.game_over_text {
            game_over_text.set_text("GAME OVER".to_string());
            game_over_text.render(&mut context.renderer);
        }

        SceneResult::None
    }

    fn process(&mut self, _context: &mut Context, _elapsed: f64) -> SceneResult {
        SceneResult::None
    }

    fn get_bounds(&self) -> Rect {
        self.bounds
    }
}
