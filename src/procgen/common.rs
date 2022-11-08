use super::{XStart, YStart};
use atrl_utils::Random;

pub fn random_start_position(rng: &mut Random) -> (XStart, YStart) {
    let xroll = rng.prng.roll_dice(1, 3);
    let x = match xroll {
        1 => XStart::Left,
        2 => XStart::Center,
        _ => XStart::Right,
    };

    let yroll = rng.prng.roll_dice(1, 3);
    let y = match yroll {
        1 => YStart::Bottom,
        2 => YStart::Center,
        _ => YStart::Top,
    };

    (x, y)
}
