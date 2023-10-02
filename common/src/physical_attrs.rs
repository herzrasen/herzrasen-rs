use rand::Rng;
use struct_iterable::Iterable;

use crate::{ATTR_RANGE, Random};

#[derive(Clone, PartialEq, Debug)]
struct PhysicalAttr {
    value: f32,
}

impl From<f32> for PhysicalAttr {
    fn from(value: f32) -> Self {
        PhysicalAttr { value }
    }
}

impl Random for PhysicalAttr {
    fn random() -> Self {
        let mut rng = rand::thread_rng();
        let value: f32 = rng.gen_range(ATTR_RANGE);
        PhysicalAttr { value }
    }
}

#[derive(Clone, PartialEq, Debug, Iterable)]
pub struct PhysicalAttrs {
    acceleration: PhysicalAttr,
    agility: PhysicalAttr,
    balance: PhysicalAttr,
    jumping: PhysicalAttr,
    injury_proneness: PhysicalAttr,
    pace: PhysicalAttr,
    stamina: PhysicalAttr,
    strength: PhysicalAttr,
}

impl Random for PhysicalAttrs {
    fn random() -> Self {
        PhysicalAttrs {
            acceleration: PhysicalAttr::random(),
            agility: PhysicalAttr::random(),
            balance: PhysicalAttr::random(),
            jumping: PhysicalAttr::random(),
            injury_proneness: PhysicalAttr::random(),
            pace: PhysicalAttr::random(),
            stamina: PhysicalAttr::random(),
            strength: PhysicalAttr::random(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_random_attr() {
        let attr = PhysicalAttr::random();
        assert_in_range(&attr)
    }

    #[test]
    fn test_create_random_physical_attrs() {
        let attrs = PhysicalAttrs::random();
        for (k, v) in attrs.iter() {
            if let Some(attr) = v.downcast_ref::<PhysicalAttr>() {
                assert_in_range(attr);
            } else {
                panic!("making a PhysicalAttr for {} failed", k)
            }
        }
    }

    fn assert_in_range(attr: &PhysicalAttr) {
        assert!(attr.value >= ATTR_RANGE.start);
        assert!(attr.value <= ATTR_RANGE.end);
    }
}
