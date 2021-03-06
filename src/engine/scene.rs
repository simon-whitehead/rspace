extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Rect;

use ::engine::context::Context;

pub enum SceneResult {
    None,
    Quit,
    ChangeScene(Box<Scene>)
}

pub trait Scene {
    fn init(&mut self, context: &mut Context);

    fn render(&mut self, context: &mut Context, elapsed: f64) -> SceneResult;
    fn process(&mut self, context: &mut Context, elapsed: f64) -> SceneResult;

    fn get_bounds(&self) -> Rect;
}

pub struct DefaultScene {
    bounds: Rect
}

impl DefaultScene {
    pub fn new() -> DefaultScene {
        DefaultScene {
            bounds: Rect::new(0, 0, 0, 0)
        }
    }
}

impl Scene for DefaultScene {
    fn init(&mut self, _context: &mut Context) {
        
    }

    fn render(&mut self, context: &mut Context, _elapsed: f64) -> SceneResult {
        if context.event_handler.quit || context.event_handler.key_pressed(sdl2::keyboard::Keycode::Escape) {
            return SceneResult::Quit;
        }
        
        context.renderer.set_draw_color(Color::RGB(0, 153, 204));
        context.renderer.clear();

        SceneResult::None
    }

    fn process(&mut self, _context: &mut Context, _elapsed: f64) -> SceneResult {
        SceneResult::None
    }

    fn get_bounds(&self) -> Rect {
        self.bounds
    }
}

pub struct FrameTimer {
    pub elapsed: f64,
    pub fps: u32,
    pub interval: u32,
    pub prev: u32,
    pub last_second: u32,

    pub last_fps: u32,   // The last captured FPS we had
    pub ticks: u32
}

impl FrameTimer {
    pub fn new(
        interval: u32,
        previous: u32,
        last_second: u32,
        fps: u32) -> FrameTimer {

        FrameTimer {
            interval: interval,
            prev: previous,
            last_second: last_second,
            fps: fps,
            elapsed: 0f64,

            last_fps: 0,
            ticks: 0
        }
    }
}
