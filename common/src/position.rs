use rand::Rng;

use crate::position::Position::{Defender, Goalkeeper, Midfielder, Striker};
use crate::Random;

#[derive(PartialEq, Clone, Debug)]
pub enum Position {
    Goalkeeper,
    Defender,
    Midfielder,
    Striker,
}

impl Random for Position {
    fn random() -> Self {
        let mut rnd = rand::thread_rng();
        match rnd.gen_range(0..4) {
            0 => Goalkeeper,
            1 => Defender,
            2 => Midfielder,
            _ => Striker,
        }
    }
}
