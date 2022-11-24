use crate::prelude::*;

pub fn load_game_textures(mut commands: Commands, asset_server: Res<AssetServer,>,) {
    commands.insert_resource(TextureAssets {
        button: asset_server.load("images/ui/button.png",),
        logo: asset_server.load("images/ui/atrl_logo.png",),
        ui_panel: asset_server.load("images/ui/panel1.png",),
        button_hover: asset_server.load("images/ui/button-hover.png",),
    },);
}
