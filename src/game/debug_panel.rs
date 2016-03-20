extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Renderer;

use ::engine::context::Context;
use ::engine::text::Text;

use ::game::levels::OpCode;

pub struct DebugPanel {
    panel: Option<Text>,
    active_explosions: u32,
    enemies: u32,
    bullets: u32,

    level: u32,
    level_opcode: OpCode,

    player_health: i32
}

impl DebugPanel {
    pub fn new() -> DebugPanel {
        DebugPanel {
            panel: None,
            active_explosions: 0,
            enemies: 0,
            bullets: 0,
            level: 0,
            level_opcode: OpCode::None,
            player_health: 0
        }
    }

    pub fn init(&mut self, context: &mut Context, bounds: Rect) {
        let mut debug_panel = Text::new((5, 5), "DEBUG PANEL", 12, Color::RGBA(255, 255, 0, 255), "assets/fonts/OpenSans-Light.ttf", bounds);
        debug_panel.init(context);

        self.panel = Some(debug_panel);
    }

    pub fn render(&mut self, renderer: &mut Renderer) {
        let text = self.generate_text();
        if let Some(ref mut panel) = self.panel {
            panel.set_text(text);
            panel.render(renderer);
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

    pub fn set_level_info(&mut self, level: u32, opcode: OpCode) {
        self.level = level;
        self.level_opcode = opcode;
    }

    pub fn set_player_health(&mut self, health: i32) {
        self.player_health = health;
    }

    fn generate_text(&self) -> String {
        format!(
        
r"DEBUG:

Level: {}
Level OpCodes: {:?}
Active explosions: {}
Enemies: {}
Bullets: {}
Player HP: {}", 
              
              self.level,
              self.level_opcode,
              self.active_explosions,
              self.enemies,
              self.bullets,
              self.player_health
        )
    }
}
