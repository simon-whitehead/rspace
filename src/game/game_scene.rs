extern crate sdl2;
extern crate sdl2_image;

use sdl2::pixels::Color;

use engine::context::Context;
use engine::entity::Entity;
use engine::scene::{Scene, SceneResult};

pub struct GameScene {
    bounds: sdl2::rect::Rect,
    entities: Vec<Box<Entity>>
}

impl GameScene {
    pub fn new(width: u32, height: u32) -> GameScene {
        GameScene {
            bounds: sdl2::rect::Rect::new(0, 0, width, height),
            entities: Vec::new()
        }
    }
}

impl Scene for GameScene {
    fn init(&mut self, renderer: &mut sdl2::render::Renderer) {
        for entity in &mut self.entities {
            entity.init(renderer);
        }
    }

    fn render(&mut self, context: &mut Context, elapsed: f64) -> SceneResult {
        let events = &mut context.event_handler;

        if events.quit || events.key_pressed(sdl2::keyboard::Keycode::Escape) {
            return SceneResult::Quit;
        }
        
        context.renderer.set_draw_color(Color::RGB(255, 0, 0));
        context.renderer.clear();

        for entity in &mut self.entities {
            entity.render(&mut context.renderer, elapsed);
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

    fn get_bounds(&self) -> sdl2::rect::Rect {
        self.bounds
    }
}
