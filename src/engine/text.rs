extern crate sdl2;
extern crate sdl2_ttf;

use std::path::Path;

use sdl2::render::{Texture, TextureQuery};
use sdl2::pixels::Color;

use ::engine::entities::Entity;

pub struct Text<'text> {
    top: i32,
    left: i32,
    
    text: String,
    font_size: u16,
    font_path: &'text Path,
    color: Color,
    
    texture: Option<Texture>,
    bounds: sdl2::rect::Rect
}

impl<'text> Text<'text> {
    pub fn new(text: &str, top: i32, left: i32, size: u16, color: Color, path: &'text Path, bounds: sdl2::rect::Rect) -> Text<'text> {
        Text {
            top: top,
            left: left,
           
            text: text.to_string(),
            font_size: size,
            font_path: path,
            color: color,
            
            texture: None,
            bounds: bounds
        } 
    }
}

impl<'text> Entity for Text<'text> {
    fn init(&mut self, renderer: &mut sdl2::render::Renderer) {
        let ttf_context = sdl2_ttf::init().unwrap();  
        
        // Load a font
        let font = ttf_context.load_font(&self.font_path, self.font_size).unwrap();

        //Create a surface, then create a texture from the surface to render
        let text_str: &str = &self.text[..];
        let surface = font.render(text_str)
            .blended(Color::RGBA(255, 0, 0, 255)).unwrap();
            
        let tex = renderer.create_texture_from_surface(&surface).unwrap();
        self.texture = Some(tex);  
    }
    
    fn render(&mut self, renderer: &mut sdl2::render::Renderer, elapsed: f64) {
        match self.texture {
            Some(ref tex) => {
                let TextureQuery { width, height, .. } = tex.query();
                renderer.copy(&tex, Some(self.bounds), Some(sdl2::rect::Rect::new(self.left, self.top, width, height)));    
            }
            _ => ()
        }
    }
     
    fn process(&mut self, event_handler: &mut ::engine::events::Events, elapsed: f64) {
         
    }
}