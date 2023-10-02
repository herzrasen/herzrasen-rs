use crate::instruction::Instruction;
use crate::location::Location;
use crate::player::Player;
use crate::position::Position;
use crate::Random;

#[derive(PartialEq, Clone, Debug)]
pub struct ActivePlayer {
    player: Player,
    position: Position,
    instructions: Vec<Instruction>,
    location: Location,
}

impl ActivePlayer {
    pub fn new(player: Player, position: Position) -> ActivePlayer {
        ActivePlayer {
            player,
            position,
            instructions: Vec::new(),
            location: Location::root(),
        }
    }

    pub fn has_instruction(&self, instruction: &Instruction) -> bool {
        self.instructions.contains(instruction)
    }

    pub fn with_position(self, position: Position) -> ActivePlayer {
        ActivePlayer { position, ..self }
    }

    pub fn with_instruction(self, new_instruction: Instruction) -> ActivePlayer {
        let mut instructions = self.instructions;
        instructions.push(new_instruction);
        ActivePlayer {
            instructions,
            ..self
        }
    }

    pub fn move_to_location(self, new_location: Location) -> ActivePlayer {
        ActivePlayer {
            location: new_location,
            ..self
        }
    }

    pub fn distance_to_location(&self, location: &Location) -> f32 {
        self.location.distance(location)
    }
}

impl Random for ActivePlayer {
    fn random() -> Self {
        ActivePlayer {
            player: Player::random(),
            position: Position::random(),
            instructions: vec![],
            location: Location::root(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::instruction::Instruction::Kickoff;
    use crate::position::Position::Midfielder;
    use crate::Random;

    use super::*;

    #[test]
    fn test_random() {
        let active_player = ActivePlayer::random();
        println!("{:?}", active_player);
        assert_eq!(active_player.location, Location::root());
        assert_eq!(active_player.instructions.len(), 0)
    }

    #[test]
    fn test_with_position() {
        let player = Player::random();
        let active_player = ActivePlayer::new(player, Position::Goalkeeper);
        let updated = active_player.with_position(Position::Defender);
        assert_eq!(updated.position, Position::Defender);
    }

    #[test]
    fn test_with_instruction() {
        let p = Player::random();
        let ap = ActivePlayer::new(p, Midfielder).with_instruction(Instruction::Kickoff);
        assert!(ap.instructions.contains(&Kickoff))
    }

    #[test]
    fn test_with_location() {
        let p = Player::random();
        let ap = ActivePlayer::new(p, Midfielder).move_to_location(Location::new(1, 1));
        assert!(ap.location.x == 1 && ap.location.y == 1)
    }
}
