extern crate sdl2;
extern crate sdl2_image;

use std::collections;

use sdl2::pixels::Color;

use sdl2_image::LoadTexture;

use ::engine::context::Context;
use ::engine::events::Events;
use ::engine::scene::Scene;

pub struct Window<'window> {

    pub width: u32,
    pub height: u32,

    title: String,

    context: Context<'window>,
    scenes: collections::HashMap<&'static str, Box<Scene>>,
    current_scene: Box<Scene>,
    frame_timer: ::engine::scene::FrameTimer
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

        let mut timer = context.timer().unwrap();

        let interval = 1_000 / 60;
        let mut prev = timer.ticks();
        let mut last_second = timer.ticks();
        let mut fps = 0u32;

        let frame_timer = ::engine::scene::FrameTimer::new(
            interval,
            prev,
            last_second,
            fps
        );

        Window {

            width: width,
            height: height,

            title: title.to_string(),

            context: ::engine::context::Context::new(
                context,
                image_context,
                video,
                renderer,
                timer,
                events
            ),
            frame_timer: frame_timer,
            scenes: collections::HashMap::new(),
            current_scene: Box::new(::engine::scene::DefaultScene::new())
        }
    }

    pub fn process(&mut self) -> bool {
        self.context.event_handler.pump();

        self.current_scene.process(&mut self.context, self.frame_timer.elapsed);

        !(self.context.event_handler.quit || self.context.event_handler.key_pressed(sdl2::keyboard::Keycode::Escape))
    }

    pub fn render(&mut self) {
        match self.frame_cap() {
            true => {
                match self.current_scene.render(&mut self.context, self.frame_timer.elapsed) {
                    ::engine::scene::SceneResult::None => self.context.renderer.present(),
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
            println!("FPS: {}", frame_timer.fps);
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

