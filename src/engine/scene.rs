extern crate sdl2;

use sdl2::pixels::Color;

use ::engine::context::Context;
use ::engine::entity::Entity;

pub enum SceneResult {
    None,
    Quit,
    ChangeScene(Box<Scene>)
}

pub trait Scene {
    fn render(&mut self, context: &mut Context, elapsed: f64) -> SceneResult;
    fn process(&mut self, context: &mut Context, elapsed: f64) -> SceneResult;

    fn add_entity(&mut self, entity: Box<Entity>);
}

pub struct DefaultScene {
    entities: Vec<Box<Entity>>
}

impl DefaultScene {
    pub fn new() -> DefaultScene {
        DefaultScene {
            entities: Vec::new()
        }
    }
}

impl Scene for DefaultScene {
    fn render(&mut self, context: &mut Context, elapsed: f64) -> SceneResult {
        let renderer = &mut context.renderer;
        let events = &context.events;

        if events.quit || events.key_pressed(sdl2::keyboard::Keycode::Escape) {
            return SceneResult::Quit;
        }
        
        renderer.set_draw_color(Color::RGB(0, 153, 204));
        renderer.clear();

        for entity in &mut self.entities {
            entity.render(context, elapsed);
        }

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

pub struct FrameTimer {
    pub elapsed: f64,
    pub fps: u32,
    pub interval: u32,
    pub prev: u32,
    pub last_second: u32
}

impl FrameTimer {
    pub fn new(
        interval: u32,
        previous: u32,
        last_second: u32,
        fps: u32) -> FrameTimer {

        FrameTimer {
            interval: interval,
            prev: previous,
            last_second: last_second,
            fps: fps,
            elapsed: 0f64
        }
    }
}
