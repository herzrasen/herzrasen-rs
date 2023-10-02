use std::ops::RangeInclusive;

use rand::Rng;

use crate::Random;

const RANGE: RangeInclusive<u32> = 160..=195;

#[derive(Clone, PartialEq, Debug)]
pub struct Height {
    value: u32,
    unit: Unit,
}

#[derive(Clone, PartialEq, Debug)]
enum Unit {
    CENTIMETERS,
}

impl Random for Height {
    fn random() -> Self {
        let mut rng = rand::thread_rng();
        let value = rng.gen_range(RANGE);
        Height {
            value,
            unit: Unit::CENTIMETERS,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_height() {
        let height = Height::random();
        assert!(height.value > *RANGE.start());
        assert!(height.value <= *RANGE.end())
    }
}
