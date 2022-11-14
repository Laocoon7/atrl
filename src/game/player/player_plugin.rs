use crate::prelude::*;

// This is the list of "things in the game I want to be able to do based on input"
#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum PlayerAction {
    // Movement
    NorthWest,
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,

    Wait,
}

impl PlayerAction {
    // Lists like this can be very useful for quickly matching subsets of actions
    pub const DIRECTIONS: [Self; 8] = [
        Self::NorthWest,
        Self::North,
        Self::NorthEast,
        Self::East,
        Self::SouthEast,
        Self::South,
        Self::SouthWest,
        Self::West,
    ];

    pub const fn direction(self) -> Option<GridDirection> {
        match self {
            Self::NorthWest => Some(GridDirection::NorthWest),
            Self::North => Some(GridDirection::North),
            Self::NorthEast => Some(GridDirection::NorthEast),
            Self::East => Some(GridDirection::East),
            Self::SouthEast => Some(GridDirection::SouthEast),
            Self::South => Some(GridDirection::South),
            Self::SouthWest => Some(GridDirection::SouthWest),
            Self::West => Some(GridDirection::West),
            _ => None,
        }
    }
}

pub struct PlayerPlugin<T> {
    pub state_running: T,
}

impl<T: StateNext> Plugin for PlayerPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<PlayerAction>::default()).add_system_set(
            ConditionSet::new()
                .run_in_state(self.state_running.clone())
                .with_system(player_input)
                .into(),
        );

        // TODO: Remove this once states are working for player / AI
        app.add_system(
            insert_resource!(TurnState::AwaitingInput).run_if_resource_equals(TurnState::Ticking),
        );
    }
}
