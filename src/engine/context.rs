extern crate sdl2;
extern crate sdl2_image;

use sdl2::Sdl;
use sdl2::TimerSubsystem;
use sdl2::VideoSubsystem;
use sdl2::render::Renderer;
use sdl2_image::Sdl2ImageContext;
use sdl2_ttf::Sdl2TtfContext;

use ::engine::events::Events;
use ::engine::cache::TextureCache;

static DEBUG: bool = true;

pub struct Context<'window> {
    pub timer: TimerSubsystem,
    pub renderer: Renderer<'window>,
    pub event_handler: Events,
    pub texture_cache: TextureCache,
    pub ttf_context: Sdl2TtfContext,

    pub bounds: sdl2::rect::Rect,

    pub DEBUG: bool,

    context: Sdl,
    image_context: Sdl2ImageContext,
    video: VideoSubsystem,
}

impl<'window> Context<'window> {
    pub fn new(sdl_context: Sdl,
               sdl_image_context: Sdl2ImageContext,
               sdl_ttf_context: Sdl2TtfContext,
               sdl_video: VideoSubsystem,
               sdl_renderer: Renderer<'window>,
               sdl_timer: TimerSubsystem,
               event_handler: Events,
               texture_cache: TextureCache,
               bounds: sdl2::rect::Rect) -> Context<'window> {

        Context {
            context: sdl_context,
            image_context: sdl_image_context,
            ttf_context: sdl_ttf_context,
            video: sdl_video,
            renderer: sdl_renderer,
            timer: sdl_timer,
            event_handler: event_handler,
            texture_cache: texture_cache,

            bounds: bounds,

            DEBUG: DEBUG
        }
    }
}
