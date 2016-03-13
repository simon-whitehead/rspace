
/// AssetCacheResult represents a cached assets
/// physical position within the cache
pub struct AssetCacheResult {
    pub index: u32,
    pub length: u32
}

impl<'a> AssetCacheResult {
    pub fn new(index: u32, length: u32) -> AssetCacheResult {
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
