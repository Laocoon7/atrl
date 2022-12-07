use crate::prelude::*;

const RAW_MOBS_FILE: &str = "assets/raws/mobs.ron";
const RAW_PLAYER_FILE: &str = "assets/raws/player.ron";

#[derive(Debug)]
pub struct Raws {
    pub player: RawActor,
    pub mobs: Vec<RawActor>,
}

#[derive(Resource)]
pub struct RawMaster {
    raws: Raws,
    mob_indices: HashMap<String, usize>,
}

impl FromWorld for RawMaster {
    fn from_world(_world: &mut World) -> Self {
        let player = AtrlFileUtils::read_ron(RAW_PLAYER_FILE).unwrap();
        let mobs = AtrlFileUtils::read_ron(RAW_MOBS_FILE).unwrap();

        let mut master = Self::new(player, mobs);
        let mut used_names: HashSet<String> = HashSet::new();

        // Hostiles
        Self::load_entity_data(&master.raws.mobs, &mut master.mob_indices, &mut used_names);

        master
    }
}

impl RawMaster {
    #[inline(always)]
    pub fn new(player: RawActor, mobs: Vec<RawActor>) -> Self {
        Self {
            raws: Raws { player, mobs },
            mob_indices: HashMap::new(),
        }
    }

    #[inline(always)]
    pub const fn get_raws(&self) -> &Raws { &self.raws }

    pub fn get_mob(&self, mob_key: &str) -> Option<&RawActor> {
        self.mob_indices.get(mob_key).map(|i| &self.raws.mobs[*i])
    }

    pub fn load_entity_data<T: 'static + BaseRawComponent>(
        raws: &[T],
        entity_index: &mut HashMap<String, usize>,
        used_names: &mut HashSet<String>,
    ) {
        for (i, entity) in raws.iter().enumerate() {
            let entity_name = entity.name();

            if used_names.contains(&entity_name) {
                warn!("WARNING - duplicate entity name in raws [{entity_name}]");
            }

            entity_index.insert(entity_name.clone(), i);
            used_names.insert(entity_name.clone());
        }
    }
}
