use crate::{prelude::*, Noise, Prht, Prng};

#[derive(Serialize, Deserialize)]
pub struct Random {
    pub prng: Prng,
    pub prht: Prht,
    pub noise: Noise,
}

#[allow(dead_code)]
impl Random {
    pub fn new(seed: u64) -> Self {
        let mut prng = Prng::new(seed);
        let prht = Prht::new(prng.next_u64());
        let noise = Noise::new(prng.next());

        Self { prng, prht, noise }
    }

    pub fn from_entropy() -> Self { Self::new(Prng::entropy_u64()) }
}

impl Default for Random {
    fn default() -> Self { Self::from_entropy() }
}
