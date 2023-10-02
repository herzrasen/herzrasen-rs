use common::active_player::ActivePlayer;

#[derive(Clone, PartialEq)]
pub enum Event {
    KickoffExecuted(ActivePlayer),
}
