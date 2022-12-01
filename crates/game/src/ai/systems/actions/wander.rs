use std::{collections::VecDeque, ops::RangeInclusive};

use bevy::render::once_cell::sync::Lazy;
use big_brain::actions::ActionState;
use rand::distributions::Uniform;

use crate::prelude::*;

static WANDER_RANGE: Lazy<Uniform<u32>> = Lazy::new(|| Uniform::new_inclusive(3, 10));

// could be used for temporary storage for multi turn actions
#[derive(Debug, Reflect, Default, Component, Clone, Eq, PartialEq)]
#[reflect(Component)]
pub struct Wander {
    my_previous_location: UVec2,
    destination: Option<UVec2>,
}

pub fn wander_action(
    tilesets: Tilesets,
    mut commands: Commands,
    manager: Res<MapManager>,
    mut ctx: ResMut<GameContext>,
    mut target_q: Query<&mut TargetVisualizer>,
    mut action_q: Query<(&Actor, &mut ActionState, &mut Wander, &ActionSpan)>,
    mut spatial_q: Query<
        (
            &WorldPosition,
            &LocalPosition,
            &Movement,
            &Name,
            &mut AIComponent,
        ),
        Without<Player>,
    >,
) {
    use ActionState::*;

    for (Actor(actor), mut action_state, mut wander, span) in action_q.iter_mut() {
        let _guard = span.span().enter();

        let rng = ctx.random.get_prng().as_rngcore();
        let Some(map) = manager.get_current_map() else {
            error!("No map found");
            return
        };

        let Ok((ai_world_position, ai_local_position, movement,name, mut ai_component)) =
        spatial_q.get_mut(*actor) else {
                error!("Actor must have spatial components");
                return
            };

        match *action_state {
            // Init | Success | Failure
            Init | Success | Failure => {
                // Nothing to do here
                continue;
            },
            Cancelled => {
                ai_component.preferred_action = None;
                *action_state = Failure;

                if let Ok(mut target_visualizer) = target_q.get_mut(*actor) {
                    target_visualizer.clear(&mut commands);
                }

                continue;
            },

            // These two states will fall through to execution
            Requested => {
                info!("{} gonna start wandering!", name);
                *action_state = Executing;
            },
            Executing => {},
        }

        let destination = match std::mem::take(&mut wander.destination) {
            Some(destination) => {
                if Line::new(ai_local_position.0, destination).get_count() <= 1 {
                    generate_wander_path(rng, map, ai_local_position.0, movement.0).as_uvec2()
                } else {
                    destination
                }
            },
            None => generate_wander_path(rng, map, ai_local_position.0, movement.0).as_uvec2(),
        };

        wander.destination = Some(destination);
        wander.my_previous_location = ai_local_position.0;
        ai_component.preferred_action = Some(ActionType::Movement((ai_world_position.0, destination)));

        if let Ok(mut target_visualizer) = target_q.get_mut(*actor) {
            target_visualizer.update(
                &mut commands,
                &tilesets,
                ai_local_position.0,
                destination,
                Color::WHITE,
            );
        }
    }
}

fn generate_wander_path(rng: &mut impl RngCore, map: &Map, ai_pos: UVec2, movement_type: u8) -> IVec2 {
    let wander_radius = WANDER_RANGE.sample(rng);
    let wander_circle = Circle::new(ai_pos, wander_radius);

    loop {
        // Default to the circle center
        let new_destination = wander_circle.iter().choose(rng).unwrap_or_else(|| wander_circle.center());
        if map.can_place_actor(new_destination, movement_type) {
            return new_destination;
        }
    }
}
