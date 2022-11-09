use crate::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(key = "julia_mono", collection(typed))]
    pub julia_mono: Vec<Handle<Font>>,
}
