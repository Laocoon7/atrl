use crate::prelude::*;

#[derive(Reflect, Default,)]
pub enum AIType {
    #[default]
    Player,
    Aggressive,
    Scared,
}
