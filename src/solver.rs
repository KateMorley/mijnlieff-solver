//! Provides a function for solving Mijnlieff.

use crate::game::Game;
use crate::status::Status::{self, *};
use crate::tile::Tile::{self, *};

/// The order in which squares are analysed.
///
/// For 7 of the 8 possible first moves a winning response is to play a Pusher
/// in a central square, and central squares give more scoring opportunities
/// throughout the game, so central squares are tried first.
///
/// For the remaining first move (playing a Straight in a corner) a winning
/// response is to play a Straight in another corner. Players can continue
/// playing Straights in corners until all corners are occupied, after which
/// the first player's next move allows the second player to play in a
/// central square. So corner squares are tried next.
///
/// All other squares are then tried in order.
const SQUARES_PREFERENCE: [u8; 16] = [5, 6, 9, 10, 3, 15, 12, 0, 1, 2, 4, 7, 8, 11, 13, 14];

/// The order in which tiles are analysed.
///
/// For 7 of the 8 possible first moves a winning response is to play a Pusher
/// in a central square, so Pushers are tried first.
///
/// For the remaining first move (playing a Straight in a corner) a winning
/// response is to play a Straight in another corner. Players can continue
/// playing Straights in corners until all corners are occupied, after which
/// the first player's next move allows the second player to play in a
/// central square. So Straights are tried next.
///
/// Diagonals are tried third and Pullers ar tried last as Pullers are less
/// useful in maintaining control of the central squares early in the game.
const TILES_PREFERENCE: [Tile; 4] = [Pusher, Straight, Diagonal, Puller];

/// Recursively solves Mijnlieff from a specified game position.
///
/// The second parameter is updated with a count of the number games analysed.
pub fn solve(game: Game, games: &mut u64) -> Status {
    if game.is_over() {
        *games += 1;
        return game.get_status();
    }

    if game.player_must_pass() {
        return !solve(game.with_pass(), games);
    }

    // Assume a loss until we have found a better result.
    let mut status = Loss;

    for square in SQUARES_PREFERENCE {
        if game.is_available(square) {
            for tile in TILES_PREFERENCE {
                if game.has(tile) {
                    match solve(game.with_move(tile, square), games) {
                        Win => (),
                        Draw => status = Draw,
                        // A win can't be improved upon, so we can return early.
                        Loss => return Win,
                    }
                }
            }
        }
    }

    status
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tile::Tile;

    #[test]
    fn test_solve() {
        let mut games = 0;

        // Player can force a win:
        //
        // Player: pusher in square 10
        // Opponent: forced pass
        // Player: pusher in square 15
        // Opponent: forced pass
        // Player wins 2-1
        assert_eq!(
            Win,
            solve(create_game(Puller, Straight, Diagonal), &mut games)
        );

        // All moves lead to one of two configurations, both of which are drawn
        //
        // Player: straight in square 10/15
        // Opponent: straight in square 11/14
        // Player: forced straight in square 15/10
        // Opponent: forced straight in square 14/11
        // Drawn 3-3
        //
        // Player: straight in square 11/14
        // Opponent: straight in square 10/15
        // Player: forced straight in square 14/11
        // Opponent: forced straight in square 15/10
        // Drawn 1-1
        assert_eq!(
            Draw,
            solve(create_game(Puller, Pusher, Diagonal), &mut games)
        );

        // Opponent can force a win after any move from the player
        //
        // Player: diagonal in square 10
        // Opponent: diagonal in square 15
        // Player: forced pass
        // Opponent: diagonal in square 14
        // Player: forced diagonal in square 11
        // Opponent wins 3-2
        //
        // Player: diagonal in square 11
        // Opponent: diagonal in square 14
        // Player: forced pass
        // Opponent: diagonal in square 10
        // Player: forced diagonal in square 15
        // Opponent wins 3-2
        //
        // Player: diagonal in square 14
        // Opponent: diagonal in square 11
        // Player: forced pass
        // Opponent: diagonal in square 10
        // Player: forced diagonal in square 15
        // Opponent wins 3-1
        //
        // Player: diagonal in square 15
        // Opponent: diagonal in square 10
        // Player: forced pass
        // Opponent: diagonal in square 11
        // Player: forced diagonal in square 14
        // Opponent wins 3-1
        assert_eq!(
            Loss,
            solve(create_game(Puller, Pusher, Straight), &mut games)
        );
    }

    // Creates a the following board arrangement, with every unoccupied square
    // available for the next move:
    //
    // 1 2 1 2
    // 2 1 2 1
    // 1 2 . .
    // 1 2 . .
    //
    // The specified tiles are used, so the solver can be tested with the
    // remaining tiles. Note that this function takes advantage of the fact that
    // the validity of moves isn't checked, and it makes mostly invalid moves.
    fn create_game(tile_1: Tile, tile_2: Tile, tile_3: Tile) -> Game {
        Game::default()
            .with_move(tile_1, 0)
            .with_move(tile_1, 1)
            .with_move(tile_1, 2)
            .with_move(tile_1, 3)
            .with_move(tile_2, 7)
            .with_move(tile_2, 6)
            .with_move(tile_2, 5)
            .with_move(tile_2, 4)
            .with_move(tile_3, 8)
            .with_move(tile_3, 9)
            .with_move(tile_3, 12)
            .with_move(tile_3, 13)
            .with_pass()
            .with_pass()
    }
}
