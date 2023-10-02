use crate::active_player::ActivePlayer;
use crate::location::Location;
use crate::team_type::TeamType;

#[derive(Clone, PartialEq, Debug)]
pub enum BallAction {
    Cross,
    Pass,
    Shot,
    Dribble,
    None,
}

#[derive(Clone, PartialEq, Debug)]
pub enum BallPace {
    Slow,
    Moderate,
    Fast,
    Stopped,
}

#[derive(Clone)]
pub struct Ball {
    location: Location,
    desired_location: Option<Location>,
    controlled_by: Option<ActivePlayer>,
    action: BallAction,
    pace: BallPace,
    coin_toss_winner: TeamType,
}

impl Ball {
    pub fn new(coin_toss_winner: TeamType) -> Ball {
        Ball {
            location: Location::root(),
            desired_location: None,
            controlled_by: None,
            action: BallAction::None,
            pace: BallPace::Stopped,
            coin_toss_winner,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ball = Ball::new(TeamType::Home);
        assert_eq!(ball.coin_toss_winner, TeamType::Home);
        assert_eq!(ball.location, Location::root());
        assert_eq!(ball.action, BallAction::None);
        assert_eq!(ball.pace, BallPace::Stopped);
        assert_eq!(ball.controlled_by, None);
    }
}
