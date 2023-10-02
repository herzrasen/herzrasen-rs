use crate::location::Location;
use crate::player::Player;

enum BallAction {
    Cross,
    Pass,
    Shot,
    Dribble,
}

enum BallPace {
    Slow,
    Moderate,
    Fast,
}

pub struct Ball<'a> {
    location: Location,
    desired_location: Option<Location>,
    controlled_by: Option<&'a Player>,
    action: BallAction,
    pace: BallPace,
}
