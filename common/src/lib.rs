use std::ops::Range;

pub mod active_player;
pub mod active_team;
pub mod ball;
mod date;
mod firstname;
mod height;
pub mod instruction;
mod lastname;
mod location;
mod mental_attrs;
mod physical_attrs;
pub mod player;
pub mod position;
pub mod probability;
mod side;
mod team;
pub mod team_type;
mod technical_attrs;
mod weight;

const ATTR_RANGE: Range<f32> = 0.0..5.0;

pub trait Random {
    fn random() -> Self;
}
