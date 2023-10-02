use std::fmt::{Display, Formatter};

use rand::seq::SliceRandom;

use crate::Random;

#[derive(Clone, PartialEq, Debug)]
pub struct Firstname {
    value: String,
}

impl Display for Firstname {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Random for Firstname {
    fn random() -> Self {
        let mut rng = rand::thread_rng();
        let vowels: Vec<char> = vec!['a', 'e', 'i', 'o', 'u'];
        let consonants: Vec<char> = vec![
            'b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v',
            'w', 'x', 'y', 'z',
        ];
        let pattern = vec![
            *consonants.choose(&mut rng).unwrap(),
            *vowels.choose(&mut rng).unwrap(),
            *consonants.choose(&mut rng).unwrap(),
            *vowels.choose(&mut rng).unwrap(),
        ];
        let name: String = pattern.iter().collect();
        let cap_name = name[0..1].to_uppercase() + &name[1..];
        Firstname { value: cap_name }
    }
}

impl From<String> for Firstname {
    fn from(value: String) -> Self {
        Firstname { value }
    }
}

impl From<&str> for Firstname {
    fn from(value: &str) -> Self {
        Firstname {
            value: value.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_firstname() {
        let firstname: Firstname = Firstname::random();
        println!("{}", firstname);
        assert!(!firstname.value.is_empty())
    }

    #[test]
    fn test_firstname_from_string() {
        let firstname: Firstname = String::from("Fritz").into();
        assert_eq!(firstname.value, "Fritz")
    }

    #[test]
    fn test_firstname_from_str() {
        let firstname: Firstname = "Fritz".into();
        assert_eq!(firstname.value, "Fritz")
    }
}
