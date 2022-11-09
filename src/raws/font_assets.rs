use crate::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/JuliaMono/JuliaMono-Regular.ttf")]
    pub julia_mono: Handle<Font>,

    #[asset(path = "fonts/JuliaMono/JuliaMono-Medium.ttf")]
    pub julia_mono_medium: Handle<Font>,

    #[asset(path = "fonts/JuliaMono/JuliaMono-Bold.ttf")]
    pub julia_mono_bold: Handle<Font>,
}
