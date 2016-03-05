extern crate sdl2;

use sdl2::pixels::Color;

use ::engine::events::Events;
use ::engine::context::Context;

pub struct Window<'window> {
    title: String,

    context: Context<'window>
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

            context: ::engine::context::Context::new(
                context,
                video,
                renderer,
                events
            )
        }
    }

    pub fn process(&mut self) -> bool {
        self.context.events.pump();

        !(self.context.events.quit || self.context.events.key_escape)
    }

    pub fn render(&mut self) {
        self.context.renderer.set_draw_color(Color::RGB(0, 153, 204));
        self.context.renderer.clear();
        self.context.renderer.present();
    }
}

