extern crate sdl2;
extern crate sdl2_image;
extern crate sdl2_ttf;

use sdl2::pixels::Color;

use ::engine::cache::TextureCache;
use ::engine::context::Context;
use ::engine::events::Events;
use ::engine::scene::{DefaultScene, FrameTimer, Scene, SceneResult};
use ::engine::text::Text;

pub struct Window<'window> {

    pub width: u32,
    pub height: u32,
    pub context: Context<'window>,

    current_scene: Box<Scene>,
    fps: Option<::engine::text::Text>,
    frame_timer: FrameTimer
}

impl<'window> Window<'window> {
    pub fn new(title: &str, width: u32, height: u32) -> Window<'window> {
        
        let context = sdl2::init().unwrap();
        let image_context = sdl2_image::init(sdl2_image::INIT_PNG).unwrap();
        let ttf_context = sdl2_ttf::init().unwrap();

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
        let prev = timer.ticks();
        let last_second = timer.ticks();
        let fps = 0u32;

        let frame_timer = FrameTimer::new(
            interval,
            prev,
            last_second,
            fps
        );

        Window {

            width: width,
            height: height,

            context: Context::new(
                context,
                image_context,
                ttf_context,
                video,
                renderer,
                timer,
                events,
                texture_cache,
                sdl2::rect::Rect::new(0, 0, width, height)
            ),
            frame_timer: frame_timer,
            fps: None,
            current_scene: Box::new(DefaultScene::new())
        }
    }

    pub fn init(&mut self) {
        self.current_scene.init(&mut self.context);
        
        // Initialize and store a Text entity that will draw the current FPS
        let mut fps = Text::new((800-48, 10), "0", 24, Color::RGBA(255, 255, 0, 255), "assets/fonts/OpenSans-Bold.ttf", self.current_scene.get_bounds());
                
        fps.init(&mut self.context);
        self.fps = Some(fps);
    }

    pub fn process(&mut self) -> bool {
        // Pump the events through the event pump
        self.context.event_handler.pump();

        // Let our current scene update
        match self.current_scene.process(&mut self.context, self.frame_timer.elapsed) {
            SceneResult::ChangeScene(mut scene) => {
                scene.init(&mut self.context);
                self.current_scene = scene; 
            },
            _ => ()
        }
        
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
                            fps.render(&mut self.context.renderer);
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

        frame_timer.ticks = now;

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
}

