use crate::prelude::*;

pub(crate) trait PathAlgorithm {
    fn compute_path(
        origin: IVec2,
        destination: IVec2,
        movement_type: u8,
        provider: &impl PathProvider,
    ) -> Option<Vec<IVec2>>;
}
