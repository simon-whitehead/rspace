
extern crate sdl2;
extern crate sdl2_image;

use std::path::Path;

use sdl2::rect::Rect;
use sdl2::render::{Renderer, Texture, TextureQuery};

use ::engine::cache::{TextureCache, AssetCacheResult};
use ::engine::context::Context;
use ::engine::events::Events;

/// FrameAnimatedSprite represents a sprite that animates via
/// frames stored as individual files.
pub struct FrameAnimatedSprite {
    top: i32,
    left: i32,

    width: u32,
    height: u32,
    
    cache: AssetCacheResult,
    frame_delay: f64,
    current_time: f64,

    bounds: Rect
}

impl FrameAnimatedSprite {
    pub fn new(position: (i32, i32),
               frame_delay: f64,
               bounds: Rect,
               cache_result: AssetCacheResult) -> FrameAnimatedSprite {

        FrameAnimatedSprite {
            left: position.0,
            top: position.1,

            width: 0,
            height: 0,

            cache: cache_result,
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

    pub fn render(&mut self, texture_cache: &TextureCache, renderer: &mut Renderer, elapsed: f64) {
        // Calculate the frame offset we are currently rendering
        let current_frame =
            (self.current_time / self.frame_delay) as usize % self.cache.length as usize;

        // Grab the frame from the texture cache, with the cache offset applied
        let sprite = &texture_cache.assets[self.cache.index as usize + current_frame as usize];

        renderer.copy(sprite, Some(self.bounds), Some(Rect::new(self.left, self.top, self.width, self.height)));
    }

    pub fn process(&mut self, event_handler: &mut Events, elapsed: f64) {
        // Update the time
        self.current_time += elapsed;
    }
}
