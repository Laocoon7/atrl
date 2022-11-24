use crate::prelude::*;
pub trait PathProvider {
    fn is_walkable(&self, position: IVec2, movement_type: u8) -> bool;
    fn cost(&self, position: IVec2, movement_type: u8) -> u32;
}
