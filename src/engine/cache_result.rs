
/// AssetCacheResult represents a cached assets
/// physical position within the cache
pub struct AssetCacheResult {
    pub index: u32,
    pub length: u32,
    pub width: u32,
    pub height: u32
}

impl<'a> AssetCacheResult {
    pub fn new(index: u32, length: u32, width: u32, height: u32) -> AssetCacheResult {
        AssetCacheResult {
            index: index,
            length: length,

            width: width,
            height: height
        }
    }
}

impl Clone for AssetCacheResult {
    fn clone(&self) -> Self {
        AssetCacheResult {
            index: self.index,
            length: self.length,

            width: self.width,
            height: self.height
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.index = source.index;
        self.length = source.length;

        self.width = source.width;
        self.height = source.height;
    }
}
