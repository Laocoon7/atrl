use crate::prelude::*;

#[derive(Deserialize, Clone, Debug)]
pub struct RawMob {
    pub ai: AIType,
    pub actor: RawActor,
}

impl BaseRawComponent for RawMob {
    fn name(&self) -> String { self.actor.name.clone() }

    fn as_any(&self) -> &dyn std::any::Any { self }
}
