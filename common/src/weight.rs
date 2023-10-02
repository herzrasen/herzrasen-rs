use std::ops::RangeInclusive;

use rand::Rng;

use crate::Random;

const RANGE: RangeInclusive<f32> = 55.0..=95.0;

#[derive(PartialEq, Debug, Clone)]
pub struct Weight {
    value: f32,
    unit: Unit,
}

#[derive(Debug, Clone, PartialEq)]
enum Unit {
    KILOGRAM,
    GRAM,
}

impl Random for Weight {
    fn random() -> Self {
        let mut rng = rand::thread_rng();
        let value = rng.gen_range(RANGE);
        Weight {
            value,
            unit: Unit::KILOGRAM,
        }
    }
}

impl Weight {
    fn convert_to(&self, unit: Unit) -> Weight {
        match self.unit {
            Unit::KILOGRAM if unit == Unit::GRAM => Weight {
                unit: Unit::GRAM,
                value: self.value * 1000.0,
            },
            Unit::KILOGRAM => self.clone(),
            Unit::GRAM if unit == Unit::KILOGRAM => Weight {
                unit: Unit::KILOGRAM,
                value: self.value / 1000.0,
            },
            Unit::GRAM => self.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_random_weight() {
        let weight = Weight::random();
        assert_eq!(weight.unit, Unit::KILOGRAM);
        assert!(weight.value > *RANGE.start());
        assert!(weight.value <= *RANGE.end())
    }

    #[test]
    fn test_convert_to_other_unit() {
        let weight = Weight {
            unit: Unit::GRAM,
            value: 85500.0,
        };
        let conv = weight.convert_to(Unit::KILOGRAM);
        assert_eq!(conv.unit, Unit::KILOGRAM);
        assert_eq!(conv.value, 85.5)
    }

    #[test]
    fn test_convert_to_same_unit() {
        let weight = Weight {
            unit: Unit::GRAM,
            value: 85500.0,
        };
        let conv = weight.convert_to(Unit::GRAM);
        assert_eq!(conv.unit, Unit::GRAM);
        assert_eq!(conv.value, 85500.0)
    }
}
