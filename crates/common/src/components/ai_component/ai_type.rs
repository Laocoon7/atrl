use crate::prelude::*;

#[derive(Reflect, Default)]
pub enum AIType {
    #[default]
    Human,
    Aggressive,
    Scared,
}
