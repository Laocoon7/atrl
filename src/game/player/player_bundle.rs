use crate::game::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,

    #[bundle]
    pub actor: ActorBundle,
    #[bundle]
    pub input_manager: InputManagerBundle<PlayerAction>,
}

impl PlayerBundle {
    pub fn default_input_map() -> InputMap<PlayerAction> {
        // This allows us to replace `ArpgAction::Up` with `Up`,
        // significantly reducing boilerplate
        use PlayerAction::*;
        let mut input_map = InputMap::default();

        // Movement
        input_map.insert(KeyCode::Up, Up);
        input_map.insert(GamepadButtonType::DPadUp, Up);

        input_map.insert(KeyCode::Down, Down);
        input_map.insert(GamepadButtonType::DPadDown, Down);

        input_map.insert(KeyCode::Left, Left);
        input_map.insert(GamepadButtonType::DPadLeft, Left);

        input_map.insert(KeyCode::Right, Right);
        input_map.insert(GamepadButtonType::DPadRight, Right);

        input_map
    }
}
