use crate::prelude::*;

pub const TURN_TIME: u32 = 1000;

#[derive(Default, Resource)]
pub struct TurnManager {
    turn_number: u32,
    current_time: u32,
    entities: IndexList<(u32, u32, Entity)>,
}

impl TurnManager {
    // This will go into an `impl FromWorld`
    // FIX: Does it need to?? - Jacob
    // Yes, as this needs to be serialized,
    // and Entity is not reflectable. We will
    // have to match new entity id's to the stored
    // entities. FromWorld allows us access to all
    // the resources and entities already generated.
    // Serialization is going to be a pain! XD

    // pub fn new() -> Self {
    //     Self {
    //         turn_number: 0,
    //         current_time: 0,
    //         entities: IndexList::new(),
    //     }
    // }

    /// Add entities to the TurnManager when building the map.
    pub fn add_entity(&mut self, entity: Entity) {
        if let Some((turn_number, current_time, _entity)) = self.entities.get_last() {
            self.entities.insert_last((*turn_number, *current_time, entity));
        } else {
            self.entities.insert_last((self.turn_number, self.current_time, entity));
        }
    }

    /// Remove an entity when it dies or the map unloads.
    pub fn remove_entity(&mut self, entity: Entity) {
        let index = self.entities.first_index();
        let mut found_index = None;
        while index.is_some() {
            if let Some((_turn_number, _current_time, current_entity)) = self.entities.get(index) {
                if *current_entity == entity {
                    found_index = Some(index);
                    break;
                }
            }
        }
        if let Some(index) = found_index {
            self.entities.remove(index);
        }
    }

    /// Clear all entities
    /// possibly reset the turn_number
    pub fn clear_entities(&mut self, reset_turn_number: bool) {
        if reset_turn_number {
            self.turn_number = 0;
        }
        self.current_time = 0;
        self.entities.clear();
    }

    /// This will get the next entity who is ready for a new turn
    /// once you get the entity from this, match it to either:
    /// ```
    /// // check this first as it's more common:
    /// if let Ok((AI_QUERY_COMPONENTS)) = q_ai_components.get_mut() {[...]}
    /// ```
    /// or
    /// ```
    /// if let Ok((PLAYER_QUERY_COMPONENTS)) = q_player_components.get_mut() {[...]}
    /// ```
    pub fn start_entity_turn(&mut self) -> Option<Entity> {
        if let Some((turn_number, current_time, entity)) = self.entities.remove_first() {
            // we are at least to turn_number:current_time
            self.turn_number = turn_number;
            self.current_time = current_time;
            return Some(entity);
        }
        None
    }

    /// After the entity performs an action,
    /// call this to re-add the entity to the queue
    /// time_spent is the amount of time used to perform the action.
    pub fn end_entity_turn(&mut self, entity: Entity, time_spent: u32) {
        let mut next_turn = self.turn_number;
        let mut next_time = self.current_time + time_spent;
        loop {
            if next_time < TURN_TIME {
                break;
            }
            next_turn += 1;
            next_time -= TURN_TIME;
        }

        if let Some(index) = self.get_index_after_time(next_turn, next_time) {
            self.entities.insert_before(index, (next_turn, next_time, entity));
        } else {
            self.entities.insert_last((next_turn, next_time, entity));
        }
    }

    fn get_index_after_time(&self, turn_number: u32, time: u32) -> Option<Index> {
        let mut index = self.entities.first_index();
        while index.is_some() {
            if let Some((current_turn_number, current_time, _entity)) = self.entities.get(index) {
                if *current_turn_number > turn_number ||
                    (*current_turn_number == turn_number && *current_time > time)
                {
                    return Some(index);
                }
            }
            index = self.entities.next_index(index);
        }
        None
    }
}

// impl FromWorld for TurnManager {
//     fn from_world(_world: &mut World) -> Self {
//         println!("TurnManager::from_world");
//         Self::new()
//     }
// }
