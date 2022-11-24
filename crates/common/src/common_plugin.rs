use crate::prelude::*;

pub struct CommonPlugin;
impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App,) { app.init_resource::<WhitePixel>(); }
}
