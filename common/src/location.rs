pub struct Location {
    pub x: i32,
    pub y: i32,
}

impl Location {
    pub fn new(x: i32, y: i32) -> Location {
        Location { x, y }
    }

    pub fn root() -> Location {
        Location::new(0, 0)
    }

    pub fn distance(&self, other: &Location) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let sum: f32 = (dx * dx + dy * dy) as f32;
        sum.sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_to_self_is_zero() {
        let loc = Location { x: 5, y: 10 };
        let dist = loc.distance(&loc);
        assert_eq!(dist, 0.0)
    }

    #[test]
    fn test_distance_to_other() {
        let loc1 = Location { x: 0, y: 0 };
        let loc2 = Location { x: -4, y: -4 };
        let dist = loc1.distance(&loc2);
        let expected = ((4 * 4 + 4 * 4) as f32).sqrt();
        assert_eq!(dist, expected)
    }
}
