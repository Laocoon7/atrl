use crate::prelude::*;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub world_position: WorldPosition,
    pub movement_component: Movement,
    pub vision_component: Vision,
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
        input_map // ArrowKeys
            .insert(KeyCode::Up, North)
            .insert(GamepadButtonType::DPadUp, North)
            .insert(KeyCode::Down, South)
            .insert(GamepadButtonType::DPadDown, South)
            .insert(KeyCode::Left, West)
            .insert(GamepadButtonType::DPadLeft, West)
            .insert(KeyCode::Right, East)
            .insert(GamepadButtonType::DPadRight, East);

        input_map // Numpad
            .insert(KeyCode::Numpad7, NorthWest)
            .insert(KeyCode::Numpad8, North)
            .insert(KeyCode::Numpad9, NorthEast)
            .insert(KeyCode::Numpad6, East)
            .insert(KeyCode::Numpad3, SouthEast)
            .insert(KeyCode::Numpad2, South)
            .insert(KeyCode::Numpad1, SouthWest)
            .insert(KeyCode::Numpad4, West);

        input_map // Waiting
            .insert(KeyCode::Numpad5, Wait);

        input_map
    }
}
