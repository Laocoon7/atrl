use super::systems::*;
use crate::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct RawPlugin<T> {
    pub state_asset_load: T,
    pub state_construct: T,
}

impl<T: StateNext> Plugin for RawPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(self.state_asset_load.clone())
                .continue_to_state(self.state_construct.clone())
                .with_collection::<TextureAssets>()
                .with_collection::<FontAssets>(),
        )
        .add_enter_system(self.state_construct.clone(), check_loaded_assets);
    }
}
