extern crate sdl2;

use sdl2::render::{Renderer, Texture};
use sdl2_image::LoadTexture;

use std;
use std::collections::HashMap;
use std::iter::FromIterator;

use ::engine::cache::AssetCacheResult;

pub struct TextureCache {
    pub assets: Vec<Texture>,
    index: usize
}

/// TextureCache holds references to assets in a Vector
impl TextureCache {
    pub fn new() -> TextureCache {
        TextureCache {
            assets: Vec::new(),
            index: 0
        }
    }

    pub fn precache(&mut self, renderer: &Renderer, path_str: &'static str) -> AssetCacheResult {
        // Store the current index
        let current_index = self.index; // Where does this group start in the cache vector

        let path = std::path::Path::new(path_str);
        let files = std::fs::read_dir(path).unwrap();
        let mut length = 0; // How many frames are in this group?

        // Iterate over the files in the directory and pull them in
        for file in files {
            let path = file.unwrap().path();
            let texture = renderer.load_texture(path.as_path()).unwrap();
            self.assets.push(texture);
            self.index += 1;
            length += 1;
        }

        // Return a view over the cached assets
        AssetCacheResult::new(current_index, length)
    }
}
