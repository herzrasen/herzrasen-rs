use crate::date::DateOfBirth;
use crate::firstname::Firstname;
use crate::height::Height;
use crate::lastname::Lastname;
use crate::mental_attrs::MentalAttrs;
use crate::physical_attrs::PhysicalAttrs;
use crate::Random;
use crate::technical_attrs::TechnicalAttrs;
use crate::weight::Weight;

#[derive(Debug)]
pub struct Player {
    lastname: Lastname,
    firstname: Firstname,
    date_of_birth: DateOfBirth,
    height: Height,
    weight: Weight,
    technical_attrs: TechnicalAttrs,
    physical_attrs: PhysicalAttrs,
    mental_attrs: MentalAttrs,
}

impl Random for Player {
    fn random() -> Self {
        Player {
            lastname: Lastname::random(),
            firstname: Firstname::random(),
            date_of_birth: DateOfBirth::random(),
            height: Height::random(),
            weight: Weight::random(),
            technical_attrs: TechnicalAttrs::random(),
            physical_attrs: PhysicalAttrs::random(),
            mental_attrs: MentalAttrs::random(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_player() {
        let player = Player::random();
        println!("{:?}", player)
    }
}