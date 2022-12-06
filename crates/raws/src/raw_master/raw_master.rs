use crate::prelude::*;

#[derive(Resource)]
pub struct RawMaster {
    raws: Raws,
}

impl RawMaster {
    #[inline(always)]
    pub const fn new(player: RawActor) -> Self {
        Self {
            raws: Raws { player },
        }
    }

    #[inline(always)]
    pub const fn get_raws(&self) -> &Raws { &self.raws }
}

impl FromWorld for RawMaster {
    fn from_world(_world: &mut World) -> Self {
        let player = Raws::load_raw::<RawActor>(RAW_PLAYER_FILE);
        Self::new(player)
    }
}
