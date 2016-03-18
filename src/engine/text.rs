extern crate sdl2;
extern crate sdl2_ttf;

use std::path::Path;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Renderer, TextureQuery};

use sdl2_ttf::Font;

use ::engine::context::Context;

pub struct Text {
    top: i32,
    left: i32,
    
    text: String,
    font: Option<Font>,
    font_size: u16,
    font_path: &'static str,
    color: Color,
    
    bounds: Rect
}

impl Text {
    pub fn new(position: (i32, i32),
               text: &str,
               size: u16,
               color: Color,
               path: &'static str,
               bounds: Rect) -> Text {

        Text {
            left: position.0,
            top: position.1,
           
            text: text.to_string(),
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

    pub fn init(&mut self, context: &Context) {
        // Load and store a font
        let font_path = Path::new(self.font_path);
        let font = context.ttf_context.load_font(&font_path, self.font_size).unwrap();
        self.font = Some(font);
    }
    
    pub fn render(&mut self, renderer: &mut Renderer) {
        if let Some(ref font) = self.font {
            // Create a surface and texture to render
            let surface = font.render(&self.text[..])
                .blended_wrapped(self.color, 200).unwrap();

            let tex = renderer.create_texture_from_surface(&surface).unwrap();

            let TextureQuery { width, height, .. } = tex.query();
            renderer.copy(&tex, Some(self.bounds), Some(Rect::new(self.left, self.top, width, height)));    
        }
    }
}
