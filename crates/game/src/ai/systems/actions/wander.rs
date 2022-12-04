use std::{collections::VecDeque, ops::RangeInclusive};

use bevy::render::once_cell::sync::Lazy;
use big_brain::actions::ActionState;
use rand::distributions::Uniform;

use crate::prelude::*;

static WANDER_RANGE: Lazy<Uniform<u32>> = Lazy::new(|| Uniform::new_inclusive(3, 10));

// could be used for temporary storage for multi turn actions
#[derive(Debug, Reflect, Component, Clone, Eq, PartialEq)]
#[reflect(Component)]
pub struct Wander {
    destination: Option<Position>,
    my_previous_location: Position,
}

impl Default for Wander {
    fn default() -> Self {
        Self {
            destination: None,
            my_previous_location: Position::new(
                WorldPosition::new(0, 0, 0),
                LocalPosition::new(0, 0, MapLayer::Actors as u32),
            ),
        }
    }
}

pub fn wander_action(
    mut commands: Commands,
    mut map_manager: MapManager,
    mut ai_context: ResMut<AiContext>,
    mut target_q: Query<&mut TargetVisualizer>,
    mut action_q: Query<(&Actor, &mut ActionState, &mut Wander)>, /* TODO: Can these be one query /
                                                                   * SystemParam? */
    mut spatial_q: Query<(&Position, &Movement, &Name, &mut AIComponent)>, // TODO: Or a ParamSet?
    player_entity: Res<PlayerEntity>,
    q_blocks_movement: Query<&BlocksMovement>,
) {
    use ActionState::*;

    for (Actor(actor), mut action_state, mut wander) in action_q.iter_mut() {
        // don't work on the player...
        if *actor == player_entity.current() {
            return;
        }

        // TODO: This is currently using the general game rng?
        // Should AI have it's own rng?
        let rng = ai_context.random.prng.as_rngcore();

        let Ok((ai_position, movement,name, mut ai_component)) =
        spatial_q.get_mut(*actor) else {
            info!("Actor must have spatial components");
                return
            };

        if ai_component.preferred_action.is_some() {
            // already wandering, quick return;
            return;
        }

        match *action_state {
            // Success | Failure
            Success | Failure => {
                // Nothing to do here
                info!("{} wander state: {:?}", name, action_state);
                return;
            },
            Cancelled => {
                info!("{} cancelled wander", name);
                ai_component.preferred_action = None;
                *action_state = Failure;

                if let Ok(mut target_visualizer) = target_q.get_mut(*actor) {
                    target_visualizer.clear(&mut commands);
                }

                return;
            },

            // These two states will fall through to execution
            Init | Requested => {
                info!("{} gonna start wandering!", name);
                *action_state = Executing;

                if let Ok(mut target_visualizer) = target_q.get_mut(*actor) {
                    target_visualizer.set_color(Color::YELLOW);
                    target_visualizer.set_style(TargetVisualizerStyle::Select);
                }
            },
            Executing => {},
        }

        info!("{} executing wander!", name);

        let destination = match std::mem::take(&mut wander.destination) {
            Some(destination) => {
                if ai_position.distance(destination) <= 1 {
                    generate_wander_path(
                        rng,
                        &mut map_manager,
                        *ai_position,
                        movement.0,
                        &q_blocks_movement,
                    )
                } else {
                    destination
                }
            },
            None => generate_wander_path(
                rng,
                &mut map_manager,
                *ai_position,
                movement.0,
                &q_blocks_movement,
            ),
        };

        wander.destination = Some(destination);
        wander.my_previous_location = *ai_position;
        ai_component.preferred_action = Some(ActionType::Movement(destination));
    }
}

fn generate_wander_path(
    rng: &mut impl RngCore,
    map: &mut MapManager,
    ai_pos: Position,
    movement_type: u8,
    q_blocks_movement: &Query<&BlocksMovement>,
) -> Position {
    let wander_radius = WANDER_RANGE.sample(rng);
    let wander_circle = Circle::new((0u32, 0), wander_radius);

    loop {
        // Default to the circle center
        let offset = wander_circle.iter().choose(rng).unwrap_or_else(|| wander_circle.center());
        let destination = ai_pos + offset;
        if map.can_place_actor(destination, movement_type, q_blocks_movement) {
            return destination;
        }
    }
}
