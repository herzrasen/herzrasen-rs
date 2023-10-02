use crate::active_player::ActivePlayer;
use crate::instruction::Instruction;
use crate::location::Location;
use crate::position::Position;
use crate::Random;
use crate::team_type::TeamType;

#[derive(Debug, Clone)]
pub struct ActiveTeam {
    lineup: Vec<ActivePlayer>,
}

impl ActiveTeam {
    fn new(lineup: Vec<ActivePlayer>) -> ActiveTeam {
        ActiveTeam { lineup }
    }

    pub fn players_with_instruction(
        &self,
        n: usize,
        instruction: &Instruction,
    ) -> Vec<&ActivePlayer> {
        self.lineup
            .iter()
            .filter(|ap| ap.has_instruction(instruction))
            .take(n)
            .collect()
    }

    pub fn closest_to_location(&self, n: usize, location: &Location) -> Vec<ActivePlayer> {
        let mut lineup = self.lineup.clone();
        lineup.sort_by(|ap1, ap2| {
            ap1.distance_to_location(location)
                .partial_cmp(&ap2.distance_to_location(location))
                .unwrap()
        });
        lineup.into_iter().take(n).collect()
    }

    pub fn random(team_type: TeamType) -> Self {
        let location_factor = if team_type == TeamType::Home { 1 } else { -1 };
        let mut lineup = Vec::new();
        let goalkeeper = ActivePlayer::random()
            .with_position(Position::Goalkeeper)
            .move_to_location(Location::new(53 * location_factor, 0));
        lineup.push(goalkeeper);
        for n in 0..4 {
            let defender = ActivePlayer::random()
                .with_position(Position::Defender)
                .move_to_location(Location::new(35 * location_factor, -30 + (n * 20)));
            lineup.push(defender)
        }
        for n in 0..4 {
            let midfielder = ActivePlayer::random()
                .with_position(Position::Midfielder)
                .move_to_location(Location::new(20 * location_factor, -30 + (n * 20)));
            lineup.push(midfielder)
        }
        for n in 0..2 {
            let striker = ActivePlayer::random()
                .with_position(Position::Striker)
                .move_to_location(Location::new(10 * location_factor, -10 + (n * 20)));
            lineup.push(striker)
        }
        ActiveTeam { lineup }
    }
}

#[cfg(test)]
mod tests {
    use crate::Random;

    use super::*;

    #[test]
    fn test_new() {
        let active_team = ActiveTeam::random(TeamType::Home);
        assert_eq!(active_team.lineup.len(), 11)
    }

    #[test]
    fn test_players_with_instructions() {
        let ap1 = ActivePlayer::random();
        let ap2 = ActivePlayer::random().with_instruction(Instruction::Kickoff);
        let lineup = vec![ap1, ap2.clone()];
        let active_team = ActiveTeam::new(lineup);
        let players = active_team.players_with_instruction(2, &Instruction::Kickoff);
        assert_eq!(players.len(), 1);
        assert_eq!(players.get(0).cloned().unwrap().clone(), ap2);
    }
}
