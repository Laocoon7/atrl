use crate::prelude::*;

#[derive(Resource, Deref, DerefMut)]
pub struct SplashTimer(pub Timer);
