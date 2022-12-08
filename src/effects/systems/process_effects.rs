use crate::prelude::*;

pub fn process_effects(mut effects: ResMut<Events<EffectType>>) {
    for effect in effects.drain() {
        match effect {
            EffectType::EntityDeath => {
                println!("EntityDeath");
            },
            EffectType::Bloodstain => {
                println!("Bloodstain");
            },
            EffectType::Damage(damage) => {
                println!("Damage: {damage}");
            },
        }
    }
}
