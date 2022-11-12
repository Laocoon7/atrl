use crate::game::prelude::*;

pub type CurrentGameState = CurrentState<GameState>;

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum AssetLoadStates {
    #[default]
    Load,
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

    Ui(UiStates),
    AssetLoad(AssetLoadStates),
    Construct(ConstructStates),
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
            Self::Initializing => Some(Self::AssetLoad(AssetLoadStates::Load)),

            // Assets
            Self::AssetLoad(asset_state) => match asset_state {
                AssetLoadStates::Load => Some(Self::Construct(ConstructStates::MapGen)),
                AssetLoadStates::LoadFailure => Some(Self::Quit),
            },

            // Construct
            Self::Construct(construct_state) => match construct_state {
                // TODO: Fix this and set mapgen => MainMenu
                ConstructStates::MapGen => Some(Self::InGame),
                // ConstructStates::MapGen => Some(Self::UiStates(UiStates::MainMenu)),
            },

            // UI
            Self::Ui(ui_state) => match ui_state {
                UiStates::MainMenu => Some(Self::InGame),
            },

            Self::InGame => Some(Self::Ui(UiStates::MainMenu)),
            Self::Quit => None,
        }
    }
}
