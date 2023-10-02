use crate::player::Player;

struct Team {
    id: String,
    name: String,
    players: Vec<Player>,
}

impl Team {
    fn new(name: String) -> Team {
        let alphabet: [char; 16] = [
            '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd', 'e', 'f',
        ];
        Team {
            id: nanoid::nanoid!(16, &alphabet),
            name,
            players: Vec::new(),
        }
    }

    fn with_player(self, new_player: Player) -> Team {
        let mut players = self.players;
        players.push(new_player);
        Team { players, ..self }
    }
}

#[cfg(test)]
mod tests {
    use crate::Random;

    use super::*;

    #[test]
    fn test_new() {
        let team = Team::new(String::from("FC Test"));
        println!("{}", team.id);
        assert_ne!(team.id, "")
    }

    #[test]
    fn test_with_player() {
        let p = Player::random();
        let team = Team::new(String::from("FC Test")).with_player(p);
        assert_eq!(team.players.len(), 1)
    }
}
