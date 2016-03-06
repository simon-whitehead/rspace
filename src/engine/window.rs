extern crate sdl2;

use std::collections;

use sdl2::pixels::Color;

use ::engine::context::Context;
use ::engine::events::Events;
use ::engine::scene::Scene;

pub struct Window<'window> {
    title: String,

    context: Context<'window>,
    scenes: collections::HashMap<&'static str, Box<Scene>>,
    current_scene: Box<Scene>
}

impl<'window> Window<'window> {
    pub fn new(title: &str, width: u32, height: u32) -> Window {
        
        let context = sdl2::init().unwrap();
        let video = context.video().unwrap();

        let events = Events::new(context.event_pump().unwrap());

        let window = video.window(title, width, height)
            .position_centered().opengl()
            .build().unwrap();

        let renderer = window.renderer()
        .accelerated()
        .build().unwrap();

        Window {
            title: title.to_string(),

            context: ::engine::context::Context::new(
                context,
                video,
                renderer,
                events
            ),
            scenes: collections::HashMap::new(),
            current_scene: Box::new(::engine::scene::DefaultScene)
        }
    }

    pub fn process(&mut self) -> bool {
        self.context.events.pump();

        !(self.context.events.quit || self.context.events.key_pressed(sdl2::keyboard::Keycode::Escape))
    }

    pub fn render(&mut self) {
        match self.current_scene.render(&mut self.context, 0f64) {
            ::engine::scene::SceneResult::None => self.context.renderer.present(),
            ::engine::scene::SceneResult::Quit => { self.context.events.quit = true; },
            _ => ()
        }
    }

    pub fn set_scene(&mut self, scene: Box<Scene>) {
        self.current_scene = scene;
    }

    pub fn add_scene(&mut self, name: &'static str, scene: Box<Scene>) {
        self.scenes.insert(name, scene);
    }
}

