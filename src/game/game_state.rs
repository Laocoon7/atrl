use crate::prelude::*;
pub type CurrentGameState = CurrentState<GameState>;

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum AssetLoadStates {
    #[default]
    Load,
    LoadCheck,
    LoadFailure,
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ConstructStates {
    #[default]
    MapGen,
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum UiStates {
    #[default]
    MainMenu,
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum GameState {
    #[default]
    Initializing,
    InGame,
    Quit,

    UiStates(UiStates),
    AssetLoadStates(AssetLoadStates),
    ConstructStates(ConstructStates),
}

/**
 * Flow:
 * 1. Initialize
 * |-> 2. SplashSetup
 * |-> 3. AssetLoadStates
 *      |-> AssetLoadFailure
 *          |-> Quit
 * |
 * |-> 4. ConstructStates
 * |-> 5. UiStates(MaiMenu)
 * |-> 6. InGame
 *      |-> MainMenu
 *      |-> Quit
 *          |-> None :)
 */

impl StateNext for GameState {
    fn next(&self) -> Option<Self> {
        match self {
            Self::Initializing => Some(Self::AssetLoadStates(AssetLoadStates::Load)),

            // Assets
            Self::AssetLoadStates(asset_state) => match asset_state {
                AssetLoadStates::Load => Some(Self::AssetLoadStates(AssetLoadStates::LoadCheck)),
                AssetLoadStates::LoadCheck => Some(Self::ConstructStates(ConstructStates::MapGen)),
                AssetLoadStates::LoadFailure => Some(Self::Quit),
            },

            // Construct
            Self::ConstructStates(construct_state) => match construct_state {
                // TODO: Fix this and set mapgen => MainMenu
                ConstructStates::MapGen => Some(Self::InGame),
                // ConstructStates::MapGen => Some(Self::UiStates(UiStates::MainMenu)),
            },

            // UI
            Self::UiStates(ui_state) => match ui_state {
                UiStates::MainMenu => Some(Self::InGame),
            },

            Self::InGame => Some(Self::UiStates(UiStates::MainMenu)),
            Self::Quit => None,
        }
    }
}
