use crate::prelude::*;

#[derive(Resource)]
pub struct RawMaster {
    raws: Raws,
    hostile_index: HashMap<String, usize>,
}

impl RawMaster {
    #[inline(always)]
    pub fn new(player: RawActor, mobs: Vec<RawMob>) -> Self {
        Self {
            raws: Raws { player, mobs },
            hostile_index: HashMap::new(),
        }
    }

    #[inline(always)]
    pub const fn get_raws(&self) -> &Raws { &self.raws }
}

impl RawMaster {
    #[inline]
    pub fn get_mob_index(&self, key: &str) -> Option<&usize> { self.hostile_index.get(key) }

    pub fn load_entity_data<T: 'static + BaseRawComponent>(
        &self,
        raws: &[T],
        entity_index: &mut HashMap<String, usize>,
        used_names: &mut HashSet<String>,
    ) {
        for (i, entity) in raws.iter().enumerate() {
            let entity_name = entity.name();

            if used_names.contains(&entity_name) {
                println!("WARNING - duplicate entity name in raws [{entity_name}]");
            }

            entity_index.insert(entity_name.clone(), i);
            used_names.insert(entity_name.clone());
        }
    }
}

impl FromWorld for RawMaster {
    fn from_world(_world: &mut World) -> Self {
        let player = Raws::load_raw::<RawActor>(RAW_PLAYER_FILE);
        let mobs = Raws::load_raw::<Vec<RawMob>>(RAW_MOBS_FILE);

        Self::new(player, mobs)
    }
}
