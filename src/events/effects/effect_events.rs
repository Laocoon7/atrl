use crate::prelude::*;

pub enum EffectType {
    EntityDeath,
    Bloodstain,
    Damage(i32),
}
