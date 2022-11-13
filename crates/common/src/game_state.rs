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
    MapGen,
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum UiState {
    #[default]
    MainMenu,
}

#[derive(Default, Resource, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum TurnState {
    #[default]
    AwaitingInput,
    Ticking,
}

#[derive(Default, Clone, Copy, PartialEq, Hash, Eq, Debug)]
pub enum GameState {
    #[default]
    Initializing,
    InGame,
    Quit,

    Ui(UiState),
    AssetLoad(AssetLoadState),
    Construct(ConstructState),
}

/**
 * Flow:
 * 1. Initialize
 * |-> 2. SplashSetup
 * |-> 3. AssetLoadState
 *      |-> AssetLoadFailure
 *          |-> Quit
 * |
 * |-> 4. ConstructState
 * |-> 5. UiState(MaiMenu)
 * |-> 6. InGame
 *      |-> MainMenu
 *      |-> Quit
 *          |-> None :)
 */

impl StateNext for GameState {
    fn next(&self) -> Option<Self> {
        match self {
            Self::Initializing => Some(Self::AssetLoad(AssetLoadState::Load)),
            Self::InGame => Some(Self::Ui(UiState::MainMenu)),
            Self::Quit => None,

            // Assets
            Self::AssetLoad(asset_state) => match asset_state {
                AssetLoadState::Load => Some(Self::Construct(ConstructState::MapGen)),
                AssetLoadState::LoadFailure => Some(Self::Quit),
            },

            // Construct
            Self::Construct(construct_state) => match construct_state {
                // TODO: Fix this and set mapgen => MainMenu
                ConstructState::MapGen => Some(Self::InGame),
                // ConstructState::MapGen => Some(Self::UiState(UiState::MainMenu)),
            },

            // UI
            Self::Ui(ui_state) => match ui_state {
                UiState::MainMenu => Some(Self::InGame),
            },
        }
    }
}

impl StateNext for TurnState {
    fn next(&self) -> Option<Self> {
        match self {
            TurnState::AwaitingInput => Some(TurnState::Ticking),
            TurnState::Ticking => Some(TurnState::AwaitingInput),
        }
    }
}

impl TurnState {
    pub fn set_next(&self, commands: &mut Commands) {
        let current = &self;

        if let Some(next) = current.next() {
            bevy::log::info!("transitioning turnstate from {:?} to {:?}", current, next);
            insert_resource!(commands, next);
        } else {
            bevy::log::error!("no next turnstate for {:?}.", current);
        }
    }
}
