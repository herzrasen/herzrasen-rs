use std::ops::Range;

mod active_player;
mod ball;
mod date;
mod firstname;
mod height;
mod instruction;
mod lastname;
mod location;
mod mental_attrs;
mod physical_attrs;
pub mod player;
mod position;
mod side;
mod technical_attrs;
mod weight;

const ATTR_RANGE: Range<f32> = 0.0..5.0;

pub trait Random {
    fn random() -> Self;
}
