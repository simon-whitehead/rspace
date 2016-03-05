extern crate sdl2;

use sdl2::{EventPump};

pub struct Events {
    pub quit: bool,
    pub keys: [bool; 323],

    pump: EventPump
}

impl Events {
    pub fn new(pump: EventPump) -> Events {
        Events {
            pump: pump,

            keys: [false; 323],
            quit: false
        }
    }

    pub fn pump(&mut self) {
        for event in self.pump.poll_iter() {
            use sdl2::event::Event::*;
            use sdl2::keyboard::Keycode::*;

            match event {
                Quit { .. } => self.quit = true,

                KeyDown { keycode, .. } => match keycode {
                    Some(k) => self.keys[k as usize] = true,
                    _ => {}
                },

                KeyUp { keycode, .. } => match keycode {
                    Some(k) => self.keys[k as usize] = false,
                    _ => {}
                },

                _ => {}
            }
        }
    }

    pub fn key_pressed(&self, keycode: sdl2::keyboard::Keycode) -> bool {
        self.keys[keycode as usize]
    }
}

