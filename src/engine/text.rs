extern crate sdl2;
extern crate sdl2_ttf;

use std::path::Path;

use sdl2::render::{Texture, TextureQuery};
use sdl2::pixels::Color;

use ::engine::cache::TextureCache;
use ::engine::context::Context;
use ::engine::entities::Entity;

pub struct Text {
    top: i32,
    left: i32,
    
    text: String,
    ttf_context: Option<sdl2_ttf::Sdl2TtfContext>,
    font: Option<sdl2_ttf::Font>,
    font_size: u16,
    font_path: &'static str,
    color: Color,
    
    bounds: sdl2::rect::Rect
}

impl Text {
    pub fn new(text: &str, top: i32, left: i32, size: u16, color: Color, path: &'static str, bounds: sdl2::rect::Rect) -> Text {
        Text {
            top: top,
            left: left,
           
            text: text.to_string(),
            ttf_context: None,
            font: None,
            font_size: size,
            font_path: path,
            color: color,
            
            bounds: bounds
        } 
    }
    
    pub fn set_text(&mut self, text: String) {
        self.text = text.to_string();
    }
}

impl Entity for Text {
    fn init(&mut self, context: &mut Context) {
        println!("Text init()");
        self.ttf_context = Some(sdl2_ttf::init().unwrap());  
        
        // Load a font
        let font_path = Path::new(self.font_path);
        if let Some(ref context) = self.ttf_context {
            let font = context.load_font(&font_path, self.font_size).unwrap();
            self.font = Some(font);
        }
    }
    
    fn render(&mut self, texture_cache: &TextureCache, renderer: &mut sdl2::render::Renderer, elapsed: f64) {
        if let Some(ref font) = self.font {
            let surface = font.render(&self.text[..])
                .blended(Color::RGBA(255, 0, 0, 255)).unwrap();

            let tex = renderer.create_texture_from_surface(&surface).unwrap();

            let TextureQuery { width, height, .. } = tex.query();
            renderer.copy(&tex, Some(self.bounds), Some(sdl2::rect::Rect::new(self.left, self.top, width, height)));    
        }
    }
     
    fn process(&mut self, event_handler: &mut ::engine::events::Events, elapsed: f64) {
         
    }
}
