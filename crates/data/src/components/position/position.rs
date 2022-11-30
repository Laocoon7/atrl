use crate::prelude::*;

#[derive(SystemParam)]
pub struct Position<'w, 's> {
    pub world_position: Query<'w, 's, &'static mut WorldPosition>,
    pub local_position: Query<'w, 's, &'static mut LocalPosition>,
}