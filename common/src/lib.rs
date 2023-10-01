use std::ops::Range;

mod date;
mod height;
mod mental_attrs;
mod lastname;
mod physical_attrs;
mod player;
mod technical_attrs;
mod weight;
mod firstname;

const ATTR_RANGE: Range<f32> = 0.0..5.0;

trait Random {
    fn random() -> Self;
}
