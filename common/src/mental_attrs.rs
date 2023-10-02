use rand::Rng;
use struct_iterable::Iterable;

use crate::{Random, ATTR_RANGE};

#[derive(Clone, PartialEq, Debug)]
struct MentalAttr {
    value: f32,
}

impl From<f32> for MentalAttr {
    fn from(value: f32) -> Self {
        MentalAttr { value }
    }
}

impl Random for MentalAttr {
    fn random() -> Self {
        let mut rng = rand::thread_rng();
        let value: f32 = rng.gen_range(ATTR_RANGE);
        MentalAttr { value }
    }
}

#[derive(Clone, PartialEq, Debug, Iterable)]
pub struct MentalAttrs {
    aggression: MentalAttr,
    anticipation: MentalAttr,
    communication: MentalAttr,
    composure: MentalAttr,
    decisions: MentalAttr,
    eccentricity: MentalAttr,
    focus: MentalAttr,
    leadership: MentalAttr,
    positioning: MentalAttr,
}

impl Random for MentalAttrs {
    fn random() -> Self {
        return MentalAttrs {
            aggression: MentalAttr::random(),
            anticipation: MentalAttr::random(),
            communication: MentalAttr::random(),
            composure: MentalAttr::random(),
            decisions: MentalAttr::random(),
            eccentricity: MentalAttr::random(),
            focus: MentalAttr::random(),
            leadership: MentalAttr::random(),
            positioning: MentalAttr::random(),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_random_attr() {
        let attr = MentalAttr::random();
        assert_in_range(&attr);
    }

    #[test]
    fn test_create_random_mental_attrs() {
        let attrs = MentalAttrs::random();
        for (k, v) in attrs.iter() {
            if let Some(attr) = v.downcast_ref::<MentalAttr>() {
                assert_in_range(attr);
            } else {
                panic!("making a MentalAttr for {} failed", k)
            }
        }
    }

    fn assert_in_range(attr: &MentalAttr) {
        assert!(attr.value >= ATTR_RANGE.start);
        assert!(attr.value <= ATTR_RANGE.end);
    }
}
