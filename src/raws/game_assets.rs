use crate::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection)]
pub struct GameAssets {
    // Any file that can be loaded and turned into a texture atlas
    #[asset(texture_atlas(tile_size_x = 8., tile_size_y = 8., columns = 16, rows = 16))]
    #[asset(path = "tilesets/terminal8x8_transparent.png")]
    pub terminal8x8_atlas: Handle<TextureAtlas>,

    // Any file that can be loaded and turned into a Image
    #[asset(path = "tilesets/terminal8x8_transparent.png")]
    pub terminal8x8: Handle<Image>,

    #[asset(path = "images/white_pixel.png")]
    pub white_pixel: Handle<Image>,
}
