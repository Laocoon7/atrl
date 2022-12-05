use crate::prelude::*;

#[derive(Bundle)]
pub struct AIBundle {
    pub ai: AIComponent,

    #[bundle]
    pub actor: ActorBundle,
}
