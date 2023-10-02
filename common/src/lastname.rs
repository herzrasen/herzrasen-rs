use rand::seq::SliceRandom;

use crate::Random;

#[derive(Clone, PartialEq, Debug)]
pub struct Lastname {
    value: String,
}

impl Random for Lastname {
    fn random() -> Self {
        let mut rng = rand::thread_rng();
        let vowels: Vec<char> = vec!['a', 'e', 'i', 'o', 'u'];
        let consonants: Vec<char> = vec![
            'b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v',
            'w', 'x', 'y', 'z',
        ];
        let pattern = vec![
            *consonants.choose(&mut rng).unwrap(),
            *consonants.choose(&mut rng).unwrap(),
            *vowels.choose(&mut rng).unwrap(),
            *consonants.choose(&mut rng).unwrap(),
        ];
        let name: String = pattern.iter().collect();
        let cap_name = name[0..1].to_uppercase() + &name[1..];
        Lastname { value: cap_name }
    }
}

impl From<String> for Lastname {
    fn from(value: String) -> Self {
        Lastname { value }
    }
}

impl From<&str> for Lastname {
    fn from(value: &str) -> Self {
        Lastname {
            value: value.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_lastname() {
        let lastname: Lastname = Lastname::random();
        println!("{:?}", lastname)
    }

    #[test]
    fn test_lastname_from_string() {
        let lastname: Lastname = String::from("Smith").into();
        assert_eq!(lastname.value, "Smith")
    }

    #[test]
    fn test_lastname_from_str() {
        let lastname: Lastname = "Smith".into();
        assert_eq!(lastname.value, "Smith")
    }
}
