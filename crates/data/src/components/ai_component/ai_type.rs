use crate::prelude::*;
#[derive(Reflect, Default, Clone, Copy)]
pub enum AIType {
    #[default]
    Player,
    Aggressive,
    Scared,
}
