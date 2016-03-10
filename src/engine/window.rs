extern crate sdl2;
extern crate sdl2_image;
extern crate sdl2_ttf;

use std::collections::HashMap;

use std::path::Path;
use sdl2::pixels::Color;

use sdl2_image::LoadTexture;

use ::engine::cache::TextureCache;
use ::engine::context::Context;
use ::engine::entities::Entity;
use ::engine::events::Events;
use ::engine::scene::{DefaultScene, FrameTimer, Scene};
use ::engine::text::Text;

pub struct Window<'window> {

    pub width: u32,
    pub height: u32,
    pub context: Context<'window>,

    title: String,

    scenes: HashMap<&'static str, Box<Scene>>,
    current_scene: Box<Scene>,
    fps: Option<::engine::text::Text>,
    frame_timer: FrameTimer
}

impl<'window> Window<'window> {
    pub fn new(title: &str, width: u32, height: u32) -> Window<'window> {
        
        let context = sdl2::init().unwrap();
        let image_context = sdl2_image::init(sdl2_image::INIT_PNG).unwrap();

        let video = context.video().unwrap();

        let events = Events::new(context.event_pump().unwrap());

        let window = video.window(title, width, height)
            .position_centered().opengl()
            .build().unwrap();

        let renderer = window.renderer()
        .accelerated()
        .build().unwrap();

        let texture_cache = TextureCache::new();

        let mut timer = context.timer().unwrap();

        let interval = 1_000 / 60;
        let mut prev = timer.ticks();
        let mut last_second = timer.ticks();
        let mut fps = 0u32;

        let frame_timer = FrameTimer::new(
            interval,
            prev,
            last_second,
            fps
        );

        Window {

            width: width,
            height: height,

            title: title.to_string(),

            context: Context::new(
                context,
                image_context,
                video,
                renderer,
                timer,
                events,
                texture_cache
            ),
            frame_timer: frame_timer,
            fps: None,
            scenes: HashMap::new(),
            current_scene: Box::new(DefaultScene::new())
        }
    }

    pub fn init(&mut self) {
        self.current_scene.init(&mut self.context);
        
        // Initialize and store a Text entity that will draw the current FPS
        let mut fps = Text::new((800-48, 10), "0", 24, Color::RGBA(255, 255, 0, 255), "assets/fonts/Lato-Thin.ttf", self.current_scene.get_bounds());
                
        fps.init(&mut self.context);
        self.fps = Some(fps);
    }

    pub fn process(&mut self) -> bool {
        // Pump the events through the event pump
        self.context.event_handler.pump();

        // Let our current scene update
        self.current_scene.process(&mut self.context, self.frame_timer.elapsed);
        
        // If we have an FPS surface, update its value
        if let Some(ref mut fps) = self.fps {
            fps.set_text(self.frame_timer.last_fps.to_string());
        }

        !(self.context.event_handler.quit || self.context.event_handler.key_pressed(sdl2::keyboard::Keycode::Escape))
    }

    pub fn render(&mut self) {
        match self.frame_cap() {
            true => {
                match self.current_scene.render(&mut self.context, self.frame_timer.elapsed) {
                    ::engine::scene::SceneResult::None => {
                        // Render the FPS on top of the scene
                        if let Some(ref mut fps) = self.fps {
                            fps.render(&self.context.texture_cache, &mut self.context.renderer, self.frame_timer.elapsed);
                        }
                        self.context.renderer.present();
                    },
                    ::engine::scene::SceneResult::Quit => { self.context.event_handler.quit = true; },
                    _ => ()
                }
            },
            false => ()
        }
    }

    fn frame_cap(&mut self) -> bool {
        let frame_timer = &mut self.frame_timer;
        let now = self.context.timer.ticks();
        let delta = now - frame_timer.prev;
        let elapsed = delta as f64 / 1_000.0;

        // Wait until 1/60th of a second has passed since we last called this
        if delta < frame_timer.interval {
            self.context.timer.delay(frame_timer.interval - delta);
            return false;
        }

        frame_timer.prev = now;
        frame_timer.fps += 1;

        frame_timer.elapsed = elapsed;

        if now - frame_timer.last_second > 1_000 {
            // Store our current FPS
            frame_timer.last_fps = frame_timer.fps;
            frame_timer.last_second = now;
            frame_timer.fps = 0;
        }

        true
    }

    pub fn set_scene(&mut self, scene: Box<Scene>) {
        self.current_scene = scene;
    }

    pub fn add_scene(&mut self, name: &'static str, scene: Box<Scene>) {
        self.scenes.insert(name, scene);
    }
}

