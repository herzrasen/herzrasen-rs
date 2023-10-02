use crate::instruction::Instruction;
use crate::location::Location;
use crate::player::Player;
use crate::position::Position;

struct ActivePlayer {
    player: Player,
    position: Position,
    instructions: Vec<Instruction>,
    location: Location,
}

impl ActivePlayer {
    fn new(player: Player, position: Position) -> ActivePlayer {
        ActivePlayer {
            player,
            position,
            instructions: Vec::new(),
            location: Location::root(),
        }
    }

    fn with_instruction(&mut self, instruction: Instruction) {
        self.instructions.push(instruction)
    }

    fn move_to_location(&mut self, new_location: Location) {
        self.location = new_location
    }
}

#[cfg(test)]
mod tests {
    use crate::instruction::Instruction::Kickoff;
    use crate::position::Position::Midfielder;
    use crate::Random;

    use super::*;

    #[test]
    fn test_with_instruction() {
        let p = Player::random();
        let mut ap = ActivePlayer::new(p, Midfielder);
        ap.with_instruction(Instruction::Kickoff);
        assert!(ap.instructions.contains(&Kickoff))
    }

    #[test]
    fn test_with_location() {
        let p = Player::random();
        let mut ap = ActivePlayer::new(p, Midfielder);
        ap.move_to_location(Location::new(1, 1));
        assert!(ap.location.x == 1 && ap.location.y == 1)
    }
}
