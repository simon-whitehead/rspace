extern crate sdl2;

use std::collections::HashMap;

use sdl2::{EventPump};
use sdl2::keyboard::Keycode;

pub struct Events {
    pub quit: bool,
    pub keys: HashMap<Keycode, bool>,

    pump: EventPump
}

impl Events {
    pub fn new(pump: EventPump) -> Events {
        Events {
            pump: pump,

            keys: HashMap::new(),
            quit: false
        }
    }

    pub fn pump(&mut self) {
        for event in self.pump.poll_iter() {
            use sdl2::event::Event::*;

            match event {
                Quit { .. } => self.quit = true,

                KeyDown { keycode, .. } => match keycode {
                    Some(k) => {
                        if let Some(k) = self.keys.get_mut(&k) {
                            *k = true;
                        }
                    },
                    _ => {}
                },

                KeyUp { keycode, .. } => match keycode {
                    Some(k) => {
                        if let Some(k) = self.keys.get_mut(&k) {
                            *k = false;
                        }
                    },
                    _ => {}
                },

                _ => {}
            }
        }
    }

    pub fn key_pressed(&mut self, keycode: sdl2::keyboard::Keycode) -> bool {
        let keys = &mut self.keys;
        if let Some(k) = keys.get(&keycode) {
            return *k;
        }

        keys.insert(keycode, false);

        false
    }
}

