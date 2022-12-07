use crate::prelude::*;
pub type CurrentGameState = CurrentState<GameState>;
#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum AssetLoadState {
    #[default]
    Load,
    LoadFailure,
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ConstructState {
    #[default]
    Construct,
    Setup,
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum UiState {
    #[default]
    MainMenu,
    Options,
}

#[derive(Default, Clone, Copy, PartialEq, Hash, Eq, Debug)]
pub enum GameState {
    #[default]
    Initializing,
    AssetLoad(AssetLoadState),
    Construct(ConstructState),
    Ui(UiState),
    InGame,
    Dead,
    Quit,
}

/// Flow:
/// 1. Initialize
/// |-> 2. SplashSetup
/// |-> 3. AssetLoadState
///      |-> AssetLoadFailure
///          |-> Quit
/// |-> 4. ConstructState
/// |-> 5. UiState(MaiMenu)
/// |-> 6. InGame
///      |-> Options
///      |-> MainMenu
///      |-> Quit
///          |-> None :)
impl StateNext for GameState {
    fn next(&self) -> Option<Self> {
        match self {
            Self::Initializing => Some(Self::AssetLoad(AssetLoadState::Load)),
            Self::InGame => Some(Self::Ui(UiState::Options)),
            Self::Quit => None,

            // Assets
            Self::AssetLoad(asset_state) => match asset_state {
                AssetLoadState::Load => Some(Self::Ui(UiState::MainMenu)),
                AssetLoadState::LoadFailure => Some(Self::Quit),
            },

            // UI
            Self::Ui(ui_state) => match ui_state {
                UiState::MainMenu => Some(Self::InGame),
                UiState::Options => Some(Self::InGame),
            },

            // Construct
            Self::Construct(construct_state) => match construct_state {
                ConstructState::Construct => Some(Self::Construct(ConstructState::Setup)),
                ConstructState::Setup => Some(Self::InGame),
                // ConstructState::Setup => Some(Self::Ui(UiState::MainMenu)),
            },

            Self::Dead => Some(Self::Ui(UiState::MainMenu)),
        }
    }
}
