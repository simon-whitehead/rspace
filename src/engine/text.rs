extern crate sdl2;
extern crate sdl2_ttf;

use std::path::Path;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Renderer, Texture, TextureQuery};

use sdl2_ttf::{Font, Sdl2TtfContext};

use ::engine::cache::TextureCache;
use ::engine::context::Context;
use ::engine::entities::Entity;
use ::engine::events::Events;

pub struct Text {
    top: i32,
    left: i32,
    
    text: String,
    ttf_context: Option<Sdl2TtfContext>,
    font: Option<Font>,
    font_size: u16,
    font_path: &'static str,
    color: Color,
    
    bounds: sdl2::rect::Rect
}

impl Text {
    pub fn new(text: &str, top: i32, left: i32, size: u16, color: Color, path: &'static str, bounds: Rect) -> Text {
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
        self.ttf_context = Some(sdl2_ttf::init().unwrap());  
        
        // Load a font
        let font_path = Path::new(self.font_path);
        if let Some(ref context) = self.ttf_context {
            let font = context.load_font(&font_path, self.font_size).unwrap();
            self.font = Some(font);
        }
    }
    
    fn render(&mut self, texture_cache: &TextureCache, renderer: &mut Renderer, elapsed: f64) {
        if let Some(ref font) = self.font {
            let surface = font.render(&self.text[..])
                .blended(Color::RGBA(255, 0, 0, 255)).unwrap();

            let tex = renderer.create_texture_from_surface(&surface).unwrap();

            let TextureQuery { width, height, .. } = tex.query();
            renderer.copy(&tex, Some(self.bounds), Some(Rect::new(self.left, self.top, width, height)));    
        }
    }
     
    fn process(&mut self, event_handler: &mut Events, elapsed: f64) {
         
    }
}
