use super::super::shared::*;
use super::astar_node::*;
use crate::prelude::*;

pub struct AStar;

impl PathAlgorithm for AStar {
    fn compute_path(
        origin: IVec2,
        destination: IVec2,
        movement_type: u8,
        partial_path_on_failure: bool,
        provider: &impl PathProvider,
    ) -> Option<Vec<IVec2>> {
        // create open/closed lists
        let mut open_nodes = IndexList::new();
        let mut closed_nodes = IndexList::new();

        // add the first node to the open list before starting the loop
        let first_node = AStarNode::new(origin, destination);
        open_nodes.insert_first(first_node);

        // loop through all the nodes
        // return if path is found
        loop {
            if open_nodes.is_empty() {
                break;
            }
            // get the lowest cost node
            if let Some(current_node) = open_nodes.remove_first() {
                if current_node.position() == destination {
                    return Self::reconstruct_path(current_node, &mut closed_nodes);
                }

                // update cardinals
                current_node.position().neighbors_cardinal().for_each(|position| {
                    current_node.update_at_position(
                        position,
                        false,
                        destination,
                        provider,
                        movement_type,
                        &mut open_nodes,
                        &mut closed_nodes,
                    );
                });

                // update ordinals
                current_node.position().neighbors_ordinal().for_each(|position| {
                    current_node.update_at_position(
                        position,
                        true,
                        destination,
                        provider,
                        movement_type,
                        &mut open_nodes,
                        &mut closed_nodes,
                    );
                });

                // close the current node
                closed_nodes.insert_last(current_node);
            }
        }
        // No path found.
        if partial_path_on_failure {
            let mut index = closed_nodes.first_index();
            let mut best_node_index = index;
            if let Some(best_node) = closed_nodes.get(best_node_index) {
                let mut best_cost = best_node.get_cost_from_end();
                index = closed_nodes.next_index(index);
                while index.is_some() {
                    if let Some(current_node) = closed_nodes.get(index) {
                        let current_cost = current_node.get_cost_from_end();
                        if best_cost > current_cost {
                            best_node_index = index;
                            best_cost = current_cost;
                            let best_node = current_node;
                        }
                    }
                    index = closed_nodes.next_index(index);
                }
            }
            if let Some(best_node) = closed_nodes.remove(best_node_index) {
                Self::reconstruct_path(best_node, &mut closed_nodes)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl AStar {
    ///This will return a path *WITHOUT* the starting point. It also
    /// does not reverse the path, so it will be in the order of last point -> first point.
    fn reconstruct_path(
        finished_node: AStarNode,
        closed_nodes: &mut IndexList<AStarNode>,
    ) -> Option<Vec<IVec2>> {
        let mut ret = Vec::new();
        let mut current_node = finished_node;

        loop {
            current_node = match current_node.get_from_node() {
                None => {
                    // ret.reverse();
                    return Some(ret);
                }
                Some(position) => {
                    ret.push(current_node.position());
                    match AStarNode::find_node_with_position(closed_nodes, position) {
                        None => return None,
                        Some(index) => closed_nodes.remove(index).unwrap(),
                    }
                }
            }
        }
    }
}
