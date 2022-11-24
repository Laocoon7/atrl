use crate::prelude::*;

pub trait FovReceiver {
    fn set_visible(&mut self, position: IVec2);
    fn get_visible(&self, position: IVec2) -> bool;
}
