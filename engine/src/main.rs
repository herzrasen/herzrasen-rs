mod r#match;
mod action;
mod event;

use common::player::*;
use common::Random;

fn main() {
    let p = Player::random();
    println!("{:?}", p)
}
