use banana_bevy_utils::state::StateNext;
use iyes_loopless::state::CurrentState;

pub type CurrentGameState = CurrentState<GameState>;

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum GameState {
    #[default]
    Initializing,
    AssetLoad,
    Construct,
    MainMenu,
    InGame,
}

impl StateNext for GameState {
    fn next(&self) -> Option<Self> {
        match self {
            GameState::Initializing => Some(GameState::AssetLoad),
            GameState::AssetLoad => Some(GameState::Construct),
            GameState::Construct => Some(GameState::InGame),
            GameState::MainMenu => Some(GameState::InGame),
            GameState::InGame => None,
        }
    }
}
