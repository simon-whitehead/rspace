extern crate sdl2;

use sdl2_image::LoadTexture;

use std;
use std::collections::HashMap;
use std::iter::FromIterator;

pub struct AssetCacheResult {
    pub index: usize,
    pub length: usize
}

impl<'a> AssetCacheResult {
    pub fn new(index: usize, length: usize) -> AssetCacheResult {
        AssetCacheResult {
            index: index,
            length: length
        }
    }
}

impl Clone for AssetCacheResult {
    fn clone(&self) -> Self {
        AssetCacheResult {
            index: self.index,
            length: self.length
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.index = source.index;
        self.length = source.length;
    }
}

pub struct TextureCache {
    pub textures: Vec<sdl2::render::Texture>,
    index: usize
}

/// TextureCache holds references to textures in a Vector
impl TextureCache {
    pub fn new() -> TextureCache {
        TextureCache {
            assets: Vec::new(),
            index: 0
        }
    }

    pub fn precache(&mut self, renderer: &sdl2::render::Renderer, path_str: &'static str) -> AssetCacheResult {
        // Store the current index
        let current_index = self.index;

        let path = std::path::Path::new(path_str);
        let files = std::fs::read_dir(path).unwrap();
        let mut length = 0;

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
