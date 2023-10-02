use crate::position::Position;
use crate::side::Side;

#[derive(PartialEq, Clone, Debug)]
pub enum Instruction {
    Role(Position),
    Kickoff,
    Corners {
        side: Side,
    },
    ThrowIn {
        side: Side,
        in_offense: bool,
        in_defense: bool,
    },
    PlayCarefully,
    PlayForward,
    AttackEarly,
    PushForward,
    TryToFinish,
    MaintainPosition,
}
