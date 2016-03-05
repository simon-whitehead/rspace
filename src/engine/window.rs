extern crate sdl2;

use sdl2::pixels::Color;

use ::engine::events::Events;

pub struct Window<'window> {
    title: String,

    events: Events,

    sdl_context: sdl2::Sdl,
    sdl_video: sdl2::VideoSubsystem,
    sdl_renderer: sdl2::render::Renderer<'window>,
}

impl<'window> Window<'window> {
    pub fn new(title: &str, width: u32, height: u32) -> Window {
        
        let context = sdl2::init().unwrap();
        let video = context.video().unwrap();

        let events = Events::new(context.event_pump().unwrap());

        let window = video.window(title, width, height)
            .position_centered().opengl()
            .build().unwrap();

        let renderer = window.renderer()
        .accelerated()
        .build().unwrap();

        Window {
            title: title.to_string(),

            events: events,

            sdl_context: context,
            sdl_video: video,
            sdl_renderer: renderer,
        }
    }

    pub fn process(&mut self) -> bool {
        self.events.pump();

        !(self.events.quit || self.events.key_escape)
    }

    pub fn render(&mut self) {
        self.sdl_renderer.set_draw_color(Color::RGB(0, 153, 204));
        self.sdl_renderer.clear();
        self.sdl_renderer.present();
    }
}

