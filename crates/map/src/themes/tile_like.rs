pub use crate::prelude::*;

pub trait TileLike {
    fn get_tileset_names(&self) -> Vec<String>;
}
