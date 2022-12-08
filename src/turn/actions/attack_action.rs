use bevy::ecs::system::CommandQueue;

use crate::prelude::*;

pub const ATTACK_TIME: u32 = SECONDS * 2; // Same as Movement, otherwise, they get another attack after player moves.

#[derive(Debug, Clone)]
pub struct AttackAction(pub Position);

impl AtrlAction for AttackAction {
    fn get_target_position(&self) -> Option<Position> { Some(self.0) }

    fn get_base_time_to_perform(&self) -> u32 { ATTACK_TIME }

    fn perform(&mut self, world: &mut World, entity: Entity) -> Result<u32, BoxedAction> {
        match try_attack(entity, self.0, world) {
            Ok(_) => Ok(self.get_base_time_to_perform()),
            Err(a) => Err(a),
        }
    }
}

pub fn try_attack(entity: Entity, position: Position, world: &mut World) -> Result<(), BoxedAction> {
    let mut system_state: SystemState<(
        MapManager,
        Query<(&mut Health, &Name)>,
        EventWriter<EffectType>,
    )> = SystemState::new(world);
    let (mut map_manager, mut health_q, mut effects_writer) = system_state.get_mut(world);

    let mut actors = Vec::new();
    let mut features = Vec::new();

    if let Some(victims) = map_manager.get_actors(position) {
        actors = victims.clone();
    }

    if let Some(victims) = map_manager.get_features(position) {
        features = victims.clone();
    }

    let mut has_attacked = false;
    for victim in actors.iter().chain(features.iter()) {
        if let Ok((mut health, name)) = health_q.get_mut(*victim) {
            has_attacked = true;
            let before = format!("{}/{}", health.current_hp, health.max_hp);
            health.current_hp -= 1;
            let after = format!("{}/{}", health.current_hp, health.max_hp);
            println!("{name} is attacking {entity:?} before: ({before}) after: ({after})");
            effects_writer.send(EffectType::Damage(1));
        }
    }

    if has_attacked {
        Ok(())
    } else {
        info!("Couldn't find entities with health components.");
        Err(WaitAction.boxed())
    }
}
