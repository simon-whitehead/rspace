extern crate sdl2;

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

