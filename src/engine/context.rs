extern crate sdl2;

pub struct Context<'window> {
    pub renderer: sdl2::render::Renderer<'window>,
    pub events: ::engine::events::Events,

    context: sdl2::Sdl,
    video: sdl2::VideoSubsystem
}

impl<'window> Context<'window> {
    pub fn new(sdl_context: sdl2::Sdl,
               sdl_video: sdl2::VideoSubsystem,
               sdl_renderer: sdl2::render::Renderer<'window>,
               events: ::engine::events::Events) -> Context {

        Context {
            context: sdl_context,
            video: sdl_video,
            renderer: sdl_renderer,
            events: events
        }
    }
}
