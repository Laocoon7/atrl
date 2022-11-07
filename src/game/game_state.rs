use std::default;

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum GameState {
    #[default]
    Initializing,
    AssetLoad,
    SplashScreen,
    _WorldGen,
    _InGame,
}
