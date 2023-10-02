use common::team_type::TeamType;

#[derive(Clone, PartialEq, Debug)]
pub enum Action {
    Kickoff(TeamType),
    ThrowIn(TeamType),
}
