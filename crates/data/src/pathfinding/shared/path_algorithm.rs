use crate::prelude::*;

pub trait PathAlgorithm {
    fn compute_path(
        origin: IVec2,
        destination: IVec2,
        movement_type: u8,
        partial_path_on_failure: bool,
        provider: &impl PathProvider,
    ) -> Option<Vec<IVec2>>;
}
