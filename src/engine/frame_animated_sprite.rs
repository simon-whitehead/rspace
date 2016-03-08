
extern crate sdl2;
extern crate sdl2_image;

use std::fs;
use std::path::Path;

use sdl2::render::{Texture, TextureQuery};
use sdl2_image::LoadTexture;

use ::engine::entities::Entity;

pub struct FrameAnimatedSprite<'sprite> {
    top: i32,
    left: i32,

    width: u32,
    height: u32,
    
    frame_path: &'sprite Path,
    frames: Vec<Texture>,
    frame_delay: f64,
    fps: f64,
    current_time: f64,

    bounds: sdl2::rect::Rect
}

impl<'sprite> FrameAnimatedSprite<'sprite> {
    pub fn new(path: &'sprite Path, frame_delay: f64, fps: f64, bounds: sdl2::rect::Rect) -> FrameAnimatedSprite {
        FrameAnimatedSprite {
            top: 0i32,
            left: 0i32,

            width: 0,
            height: 0,

            frame_path: path,
            frames: Vec::new(),
            frame_delay: frame_delay,
            fps: fps,
            current_time: 0f64,

            bounds: bounds
        }
    }
}

impl<'sprite> Entity for FrameAnimatedSprite<'sprite> {
    fn init(&mut self, renderer: &mut sdl2::render::Renderer) {
        let mut frames = Vec::new();
        let files = fs::read_dir(self.frame_path).unwrap();

        for file in files {
            let path = file.unwrap().path();
            frames.push(renderer.load_texture(path.as_path()).unwrap());
        }

        let TextureQuery { width, height, .. } = frames.first().unwrap().query();

        self.width = width;
        self.height = height;

        self.frames = frames;
    }

    fn render(&mut self, renderer: &mut sdl2::render::Renderer, elapsed: f64) {
        let current_frame =
            (self.current_time / self.frame_delay) as usize % self.frames.len();

        let sprite = &self.frames[current_frame];

        renderer.copy(sprite, Some(self.bounds), Some(sdl2::rect::Rect::new(self.left, self.top, self.width, self.height)));
    }

    fn process(&mut self, event_handler: &mut ::engine::events::Events, elapsed: f64) {
        // Update the time
        self.current_time += elapsed;
    }
}
