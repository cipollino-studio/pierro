
mod font_atlas;

use cosmic_text::{CacheKey, FontSystem, SwashCache};
use font_atlas::{FontAtlas, Glyph};

pub(crate) struct TextResources {
    pub(crate) font_system: FontSystem,
    pub(crate) swash_cache: SwashCache,
    atlas: FontAtlas
}

impl TextResources {

    pub(crate) fn new(device: &wgpu::Device) -> Self {
        let font_system = FontSystem::new(); 
        Self {
            font_system,
            swash_cache: SwashCache::new(),
            atlas: FontAtlas::new(device)
        }
    }

    pub(crate) fn get_glyph(&mut self, glyph: CacheKey, device: &wgpu::Device, queue: &wgpu::Queue) -> Option<Glyph> {
        self.atlas.get_glyph(glyph, &mut self.font_system, &mut self.swash_cache, device, queue)
    }

}
