use crate::prelude::*;
pub type CurrentGameState = CurrentState<GameState>;

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum GameState {
    #[default]
    Initializing,
    SplashScreen,
    AssetLoad,
    Construct,
    MainMenu,
    InGame,
}

impl StateNext for GameState {
    fn next(&self) -> Option<Self> {
        match self {
            Self::Initializing => Some(Self::SplashScreen),
            Self::SplashScreen => Some(Self::AssetLoad),
            Self::AssetLoad => Some(Self::Construct),
            Self::Construct => Some(Self::InGame),
            Self::MainMenu => Some(Self::InGame),
            Self::InGame => None,
        }
    }
}
