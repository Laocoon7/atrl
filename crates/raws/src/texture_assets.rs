use crate::prelude::*;

#[derive(Resource, AssetCollection)]
pub struct TextureAssets {
    #[asset(path = "images/ui/panel1.png")]
    pub ui_panel: Handle<Image>,
    #[asset(path = "images/ui/button-hover.png")]
    pub button_hover: Handle<Image>,
    #[asset(path = "images/ui/button.png")]
    pub button: Handle<Image>,
    #[asset(path = "images/ui/atrl_logo.png")]
    pub logo: Handle<Image>,
}
