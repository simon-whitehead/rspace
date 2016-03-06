extern crate sdl2;

use sdl2::pixels::Color;

use engine::context::Context;
use engine::entity::Entity;
use engine::scene::{Scene, SceneResult};

pub struct GameScene {
    entities: Vec<Box<Entity>>
}

impl GameScene {
    pub fn new() -> GameScene {
        GameScene {
            entities: Vec::new()
        }
    }
}

impl Scene for GameScene {
    fn render(&mut self, context: &mut Context, elapsed: f64) -> SceneResult {
        let renderer = &mut context.renderer;
        let events = &context.events;

        if events.quit || events.key_pressed(sdl2::keyboard::Keycode::Escape) {
            return SceneResult::Quit;
        }

        for entity in &mut self.entities {
            entity.render(context, elapsed);
        }
        
        renderer.set_draw_color(Color::RGB(255, 0, 0));
        renderer.clear();

        SceneResult::None
    }

    fn process(&mut self, context: &mut Context, elapsed: f64) -> SceneResult {
        for entity in &mut self.entities {
            entity.process(context, elapsed);
        }
        SceneResult::None
    }

    fn add_entity(&mut self, entity: Box<Entity>) {
        self.entities.push(entity);
    }
}
