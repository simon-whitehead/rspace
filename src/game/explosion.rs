extern crate sdl2;

use sdl2::render::Renderer;

use ::engine::cache::TextureCache;
use ::engine::entities::FrameAnimatedSprite;

pub struct Explosion {
    pub deleted: bool,

    x: i32,
    y: i32,

    sprite: FrameAnimatedSprite
}

impl Explosion {
    pub fn new(position: (i32, i32),
               sprite: FrameAnimatedSprite) -> Explosion {

        Explosion {
            x: position.0,
            y: position.1,

            sprite: sprite,

            deleted: false
        }
   } 

    pub fn render(&mut self, texture_cache: &TextureCache, renderer: &mut Renderer) {
        self.sprite.render((self.x, self.y), texture_cache, renderer);
    }

    pub fn process(&mut self, elapsed: f64) {
        self.sprite.process(elapsed);

        // If we've reached the end of our frames ... let the scene clean us up
        self.deleted = self.sprite.current_frame >= self.sprite.cache.length - 1;
    }
}
