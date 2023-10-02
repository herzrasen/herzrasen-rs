use rand::Rng;
use struct_iterable::Iterable;

use crate::{ATTR_RANGE, Random};

#[derive(Clone, PartialEq, Debug)]
struct TechnicalAttr {
    value: f32,
}

impl From<f32> for TechnicalAttr {
    fn from(value: f32) -> Self {
        TechnicalAttr { value }
    }
}

impl Random for TechnicalAttr {
    fn random() -> Self {
        let mut rng = rand::thread_rng();
        let value: f32 = rng.gen_range(ATTR_RANGE);
        TechnicalAttr { value }
    }
}

#[derive(Clone, PartialEq, Debug, Iterable)]
pub struct TechnicalAttrs {
    catching: TechnicalAttr,
    dribbling: TechnicalAttr,
    heading: TechnicalAttr,
    reflexes: TechnicalAttr,
    shooting: TechnicalAttr,
    throwing: TechnicalAttr,
    passing: TechnicalAttr,
    tackling: TechnicalAttr,
    technique: TechnicalAttr,
}

impl Random for TechnicalAttrs {
    fn random() -> Self {
        TechnicalAttrs {
            catching: TechnicalAttr::random(),
            dribbling: TechnicalAttr::random(),
            heading: TechnicalAttr::random(),
            reflexes: TechnicalAttr::random(),
            shooting: TechnicalAttr::random(),
            throwing: TechnicalAttr::random(),
            passing: TechnicalAttr::random(),
            tackling: TechnicalAttr::random(),
            technique: TechnicalAttr::random(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_random_attr() {
        let attr = TechnicalAttr::random();
        assert_in_range(&attr)
    }

    #[test]
    fn test_create_random_technical_attrs() {
        let attrs = TechnicalAttrs::random();
        for (k, v) in attrs.iter() {
            if let Some(attr) = v.downcast_ref::<TechnicalAttr>() {
                assert_in_range(attr);
            } else {
                panic!("making a TechnicalAttr for {} failed", k)
            }
        }
    }

    fn assert_in_range(attr: &TechnicalAttr) {
        assert!(attr.value >= ATTR_RANGE.start);
        assert!(attr.value <= ATTR_RANGE.end);
    }
}
