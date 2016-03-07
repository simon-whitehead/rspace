extern crate sdl2;
extern crate sdl2_image;

pub struct Context<'window> {
    pub timer: sdl2::TimerSubsystem,
    pub renderer: sdl2::render::Renderer<'window>,
    pub events: ::engine::events::Events,

    context: sdl2::Sdl,
    image_context: sdl2_image::Sdl2ImageContext,
    video: sdl2::VideoSubsystem
}

impl<'window> Context<'window> {
    pub fn new(sdl_context: sdl2::Sdl,
               sdl_image_context: sdl2_image::Sdl2ImageContext,
               sdl_video: sdl2::VideoSubsystem,
               sdl_renderer: sdl2::render::Renderer<'window>,
               sdl_timer: sdl2::TimerSubsystem,
               events: ::engine::events::Events) -> Context<'window> {

        Context {
            context: sdl_context,
            image_context: sdl_image_context,
            video: sdl_video,
            renderer: sdl_renderer,
            timer: sdl_timer,
            events: events
        }
    }
}
