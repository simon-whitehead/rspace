extern crate sdl2;
extern crate sdl2_image;

use sdl2::pixels::Color;
use sdl2::rect::Rect;

use engine::context::Context;
use engine::entities::Entity;
use engine::scene::{Scene, SceneResult};

pub struct GameScene {
    bounds: Rect,
    entities: Vec<Box<Entity>>
}

impl GameScene {
    pub fn new(width: u32, height: u32) -> GameScene {
        GameScene {
            bounds: Rect::new(0, 0, width, height),
            entities: Vec::new()
        }
    }
}

impl Scene for GameScene {
    fn init(&mut self, context: &mut Context) {
        for entity in &mut self.entities {
            entity.init(context);
        }
    }

    fn render(&mut self, context: &mut Context, elapsed: f64) -> SceneResult {
        let events = &mut context.event_handler;

        if events.quit || events.key_pressed(sdl2::keyboard::Keycode::Escape) {
            return SceneResult::Quit;
        }
        
        context.renderer.set_draw_color(Color::RGB(0, 0, 0));
        context.renderer.clear();

        for entity in &mut self.entities {
            entity.render(&context.texture_cache, &mut context.renderer, elapsed);
        }

        SceneResult::None
    }

    fn process(&mut self, context: &mut Context, elapsed: f64) -> SceneResult {
        for entity in &mut self.entities {
            entity.process(&mut context.event_handler, elapsed);
        }
        SceneResult::None
    }

    fn add_entity(&mut self, entity: Box<Entity>) {
        self.entities.push(entity);
    }

    fn get_bounds(&self) -> Rect {
        self.bounds
    }
}
