extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Renderer;

use ::engine::context::Context;
use ::engine::text::Text;

pub struct DebugPanel {
    panel: Option<Text>,
    active_explosions: u32,
    enemies: u32,
    bullets: u32
}

impl DebugPanel {
    pub fn new() -> DebugPanel {
        DebugPanel {
            panel: None,
            active_explosions: 0,
            enemies: 0,
            bullets: 0
        }
    }

    pub fn init(&mut self, context: &mut Context, bounds: Rect) {
        let mut debug_panel = Text::new((5, 5), "DEBUG PANEL", 12, Color::RGBA(255, 255, 0, 255), "assets/fonts/OpenSans-Light.ttf", bounds);
        debug_panel.init(context);

        self.panel = Some(debug_panel);
    }

    pub fn render(&mut self, renderer: &mut Renderer, elapsed: f64) {
        let text = self.generate_text();
        if let Some(ref mut panel) = self.panel {
            panel.set_text(text);
            panel.render(renderer, elapsed);
        }
    }

    pub fn set_active_explosions(&mut self, explosions: u32) {
        self.active_explosions = explosions;
    }

    pub fn set_enemies(&mut self, enemies: u32) {
        self.enemies = enemies;
    }

    pub fn set_bullets(&mut self, bullets: u32) {
        self.bullets = bullets;
    }

    fn generate_text(&self) -> String {
        format!(
        
r"DEBUG:

Active explosions: {}
Enemies: {}
Bullets: {}", 
              
              self.active_explosions,
              self.enemies,
              self.bullets
        )
    }
}
