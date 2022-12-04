use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct VisibilityMap {
    visible_positions: HashSet<Position>,
}

impl VisibilityMap {
    pub fn new() -> Self {
        Self {
            visible_positions: HashSet::new(),
        }
    }
}

impl FovReceiver for VisibilityMap {
    fn get_visible(&self, position: Position) -> bool {
        self.visible_positions.contains(&position)
    }

    fn set_visible(&mut self, position: Position) {
        self.visible_positions.insert(position);
    }

    fn get_all(&self) -> HashSet<Position> {
        self.visible_positions.clone()
    }
}
