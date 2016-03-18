
extern crate sdl2;
extern crate sdl2_image;

use sdl2::rect::Rect;
use sdl2::render::{Renderer, TextureQuery};

use ::engine::cache::{TextureCache, AssetCacheResult};
use ::engine::context::Context;

/// FrameAnimatedSprite represents a sprite that animates via
/// frames stored as individual files.
pub struct FrameAnimatedSprite {
    pub width: u32,
    pub height: u32,
    pub current_frame: u32,
    pub cache: AssetCacheResult,

    frame_delay: f64,
    current_time: f64,

    bounds: Rect
}

impl FrameAnimatedSprite {
    pub fn new(frame_delay: f64,
               bounds: Rect,
               cache_result: AssetCacheResult) -> FrameAnimatedSprite {

        FrameAnimatedSprite {
            width: 0,
            height: 0,

            cache: cache_result,
            current_frame: 0,
            frame_delay: frame_delay,
            current_time: 0f64,

            bounds: bounds
        }
    }

    pub fn init(&mut self, context: &mut Context) {
        // Grab a sample frame from the cache to query
        let frames = &context.texture_cache.assets[self.cache.index as usize];

        let TextureQuery { width, height, .. } = frames.query();

        // Store the width and height of the texture
        self.width = width;
        self.height = height;
    }

    pub fn render(&mut self, position: (i32, i32), texture_cache: &TextureCache, renderer: &mut Renderer) {
        // Grab the frame from the texture cache, with the cache offset applied
        let frame_index = self.cache.index + self.current_frame;
        let sprite = &texture_cache.assets[frame_index as usize];

        renderer.copy(sprite, Some(self.bounds), Some(Rect::new(position.0, position.1, self.width, self.height)));
    }

    pub fn process(&mut self, elapsed: f64) {
        // Update the time
        self.current_time += elapsed;

        // Calculate the frame offset we are currently rendering
        self.current_frame =
            ((self.current_time / self.frame_delay) % self.cache.length as f64) as u32;

    }
}
