extern crate sdl2;

use sdl2::render::{Renderer, Texture};
use sdl2_image::LoadTexture;

use std::fs;

use std::path::Path;

use ::engine::cache::AssetCacheResult;

pub struct TextureCache {
    pub assets: Vec<Texture>,
    index: u32
}

/// TextureCache holds references to assets in a Vector
impl TextureCache {
    pub fn new() -> TextureCache {
        TextureCache {
            assets: Vec::new(),
            index: 0
        }
    }

    pub fn precache_path(&mut self, renderer: &Renderer, path_str: &'static str) -> AssetCacheResult {
        // Store the current index
        let current_index = self.index; // Where does this group start in the cache vector

        let path = Path::new(path_str);
        let files = fs::read_dir(path).unwrap();
        let mut length = 0; // How many frames are in this group?

        let mut width = 0;
        let mut height = 0;

        // Iterate over the files in the directory and pull them in
        for file in files {
            let path = file.unwrap().path();
            let texture = renderer.load_texture(path.as_path()).unwrap();
            let query = texture.query();
            width = query.width;
            height = query.height;
            self.assets.push(texture);
            self.index += 1;
            length += 1;
        }

        // Return a view over the cached assets
        AssetCacheResult::new(current_index, length, width, height)
    }

    pub fn precache_file(&mut self, renderer: &Renderer, file_path: &'static str) -> AssetCacheResult {
        // Store the current index
        let current_index = self.index; // Where does this group start in the cache vector

        let path = Path::new(file_path);

        let texture = renderer.load_texture(path).unwrap();
        let query = texture.query();
        let width = query.width;
        let height = query.height;

        self.assets.push(texture);
        self.index += 1;

        // Return a view over the cached assets
        AssetCacheResult::new(current_index, 1, width, height)
    }
}
