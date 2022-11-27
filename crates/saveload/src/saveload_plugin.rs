use crate::prelude::*;
pub struct SaveLoadPlugin;
impl Plugin for SaveLoadPlugin {
    fn build(&self, app: &mut App) {
        app
            // -- Tags -- //
            .register_type::<Player>()
            // -- AI -- //
            .register_type::<AIType>()
            .register_type::<AIComponent>()
            // -- Stats -- //
            .register_type::<Health>()
            .register_type::<Equipable>()
            // -- Map -- //
            .register_type::<VisionType>()
            .register_type::<Vision>()
            .register_type::<MovementType>()
            .register_type::<Movement>()
            //.register_type::<Map>()
            // -- Position -- //
            .register_type::<WorldPosition>();
    }
}
