use std::time::Duration;

use common::active_team::ActiveTeam;
use common::ball::Ball;
use common::instruction::Instruction;
use common::probability::Probability;
use common::team_type::TeamType;
use common::team_type::TeamType::Home;

use crate::action::Action;
use crate::action::Action::{Kickoff, ThrowIn};
use crate::event::Event;

#[derive(Clone)]
struct Match {
    home_team: ActiveTeam,
    away_team: ActiveTeam,
    ball: Ball,
    next_action: Action,
    events: Vec<Event>,
    seconds_passed: Duration,
}

impl Match {
    fn new(home_team: ActiveTeam, away_team: ActiveTeam) -> Match {
        let coin_toss_winner = Probability::new(50).eval(TeamType::Home, TeamType::Away);
        let action = Kickoff(coin_toss_winner.clone());
        Match {
            home_team,
            away_team,
            ball: Ball::new(coin_toss_winner),
            next_action: action,
            events: Vec::new(),
            seconds_passed: Duration::new(0, 0),
        }
    }

    fn tick(self) -> Match {
        let seconds_passed = self.seconds_passed + Duration::from_secs(1);
        Match {
            seconds_passed,
            ..self
        }
    }

    fn next(self) -> Match {
        self.tick().eval_next_action()
    }

    fn eval_next_action(self) -> Match {
        let action = self.next_action.clone();
        match action {
            Kickoff(teamType) => self.kickoff(teamType),
            ThrowIn(team) => Match { ..self },
        }
    }

    fn kickoff(self, team_type: TeamType) -> Match {
        let kick_off_team = self.select(team_type);
        let kick_off_players = kick_off_team.players_with_instruction(2, &Instruction::Kickoff);
        if kick_off_players.len() > 2 {

        }
        let mut events = self.events.to_vec();
        events.push(Event::KickoffExecuted(
            kick_off_players.get(0).cloned().unwrap().clone(),
        ));
        Match { events, ..self }
    }

    fn select(&self, team_type: TeamType) -> &ActiveTeam {
        if team_type == Home {
            &self.home_team
        } else {
            &self.away_team
        }
    }
}

#[cfg(test)]
mod tests {
    use common::Random;
    use common::team_type::TeamType::Away;

    use super::*;

    #[test]
    fn test_new() {
        let home_team = ActiveTeam::random();
        let away_team = ActiveTeam::random();
        let ball = Ball::new(Home);
        let m = Match::new(home_team, away_team);
    }

    #[test]
    fn test_kickoff() {
        let home_team = ActiveTeam::random();
        let away_team = ActiveTeam::random();
        let ball = Ball::new(Away);
        let m = Match::new(home_team, away_team).kickoff(Away);
        assert!(m.events.len() > 0)
    }
}
