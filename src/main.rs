//! Solves Mijnlieff.

use std::time::Instant;

use game::Game;
use status::Status::*;

mod board;
mod game;
mod hand;
mod solver;
mod status;
mod tile;

/// Solves Mijnlieff and outputs the number of games analysed, the time taken,
/// and the result.
fn main() {
    let now = Instant::now();

    let mut games = 0;
    let status = solver::solve(Game::default(), &mut games);

    println!(
        "Analysed {games} games in {} seconds",
        now.elapsed().as_secs()
    );

    println!(
        "Mijnlieff is a {} with perfect play",
        match status {
            Win => "win for the first player",
            Draw => "draw",
            Loss => "win for the second player",
        }
    );
}
