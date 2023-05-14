//! Defines a struct representing the game state.

use crate::board::Board;
use crate::hand::Hand;
use crate::status::Status;
use crate::tile::Tile;

/// A bit field representing the initially unavailable squares.
///
/// For efficiency, all squares except one corner and one edge are marked as
/// unavailable, as all other first moves are rotations or reflections of moves
/// in these squares.
const INITIAL_UNAVAILABLE: u16 = 0b_1111_1111_1111_1100;

/// Represents the game state.
///
/// Only the state necessary to solve the game is stored: specifically, the
/// board of squares occupied by each player, each player's hand, and the board
/// of squares unavailable due to the previous player's move.
pub struct Game {
    /// The board of squares occupied by the current player.
    board: Board,

    /// The current player's hand.
    hand: Hand,

    /// The board of squares occupied by the opposing player.
    opponent_board: Board,

    /// The opposing player's hand.
    opponent_hand: Hand,

    /// The board of squares unavailable due to the previous player's move.
    unavailable: Board,
}

impl Default for Game {
    /// Returns the initial game state, with no squares being occupied and each
    /// player having a complete hand.
    fn default() -> Self {
        Game {
            board: Board::default(),
            hand: Hand::default(),
            opponent_board: Board::default(),
            opponent_hand: Hand::default(),
            unavailable: Board::from(INITIAL_UNAVAILABLE),
        }
    }
}

impl Game {
    /// Returns whether the game is over (because the current player's hand is
    /// empty).
    pub fn is_over(&self) -> bool {
        self.hand.is_empty()
    }

    /// Returns the victory status.
    pub fn get_status(&self) -> Status {
        let score = self.board.get_score();
        let opponent_score = self.opponent_board.get_score();

        if score > opponent_score {
            Status::Win
        } else if score < opponent_score {
            Status::Loss
        } else {
            Status::Draw
        }
    }

    /// Returns whether the player must pass (because all squares are
    /// unavailable).
    pub fn player_must_pass(&self) -> bool {
        self.unavailable.is_full()
    }

    /// Returns whether the specified square is available.
    pub fn is_available(&self, square: u8) -> bool {
        self.unavailable.is_available(square)
    }

    /// Returns whether the current player's hand contains the specified tile.
    pub fn has(&self, tile: Tile) -> bool {
        self.hand.has(tile)
    }

    /// Returns an instance for the opponent after the current player has
    /// passed. Passing allows the opponent to play in any unoccupied square.
    pub fn with_pass(&self) -> Self {
        Game {
            board: self.opponent_board,
            hand: self.opponent_hand,
            opponent_board: self.board,
            opponent_hand: self.hand,
            unavailable: Board::merge(self.board, self.opponent_board),
        }
    }

    /// Returns an instance for the opponent after the specified move.
    pub fn with_move(&self, tile: Tile, square: u8) -> Self {
        Game {
            board: self.opponent_board,
            hand: self.opponent_hand,
            opponent_board: self.board.with(square),
            opponent_hand: self.hand.without(tile),
            unavailable: Board::merge_3(
                self.board,
                self.opponent_board,
                tile.get_unavailable(square),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Game;
    use crate::board::Board;
    use crate::hand::Hand;
    use crate::status::Status::*;
    use crate::tile::Tile::*;

    #[test]
    fn is_over() {
        assert!(!Game::default().is_over());

        let hand = Hand::default()
            .without(Puller)
            .without(Puller)
            .without(Pusher)
            .without(Pusher)
            .without(Straight)
            .without(Straight)
            .without(Diagonal)
            .without(Diagonal);
        assert!(Game {
            hand,
            ..Game::default()
        }
        .is_over());
    }

    #[test]
    fn get_status() {
        assert_eq!(Draw, Game::default().get_status());

        assert_eq!(
            Win,
            Game {
                board: Board::from(0b_0000_0000_0000_0111),
                ..Game::default()
            }
            .get_status()
        );

        assert_eq!(
            Loss,
            Game {
                opponent_board: Board::from(0b_0000_0000_0000_0111),
                ..Game::default()
            }
            .get_status()
        );
    }

    #[test]
    fn player_must_pass() {
        assert!(!Game::default().player_must_pass());

        assert!(Game {
            unavailable: Board::from(0b_1111_1111_1111_1111),
            ..Game::default()
        }
        .player_must_pass())
    }

    #[test]
    fn is_available() {
        let game = Game {
            unavailable: Board::from(0b_1111_1111_1111_1110),
            ..Game::default()
        };
        assert!(game.is_available(0));
        assert!(!game.is_available(1));
    }

    #[test]
    fn has() {
        let game = Game {
            hand: Hand::default().without(Puller).without(Puller),
            ..Game::default()
        };
        assert!(!game.has(Puller));
        assert!(game.has(Pusher));
        assert!(game.has(Straight));
        assert!(game.has(Diagonal));
    }

    #[test]
    fn with_pass() {
        let game = Game {
            board: Board::from(0b_0000_0000_0000_1111),
            hand: Hand::default(),
            opponent_board: Board::from(0b_1111_0000_0000_0000),
            opponent_hand: Hand::default().without(Puller),
            ..Game::default()
        };

        let after_pass = game.with_pass();

        assert_eq!(after_pass.board, game.opponent_board);
        assert_eq!(after_pass.hand, game.opponent_hand);
        assert_eq!(after_pass.opponent_board, game.board);
        assert_eq!(after_pass.opponent_hand, game.hand);
        assert_eq!(after_pass.unavailable, Board::from(0b_1111_0000_0000_1111));
    }

    #[test]
    fn with_move() {
        let game = Game {
            board: Board::from(0b_0000_0000_0000_1111),
            hand: Hand::default(),
            opponent_board: Board::from(0b_1111_0000_0000_0000),
            opponent_hand: Hand::default().without(Puller),
            ..Game::default()
        };

        let after_move = game.with_move(Diagonal, 5);

        assert_eq!(after_move.board, game.opponent_board);
        assert_eq!(after_move.hand, game.opponent_hand);
        assert_eq!(
            after_move.opponent_board,
            Board::from(0b_0000_0000_0010_1111)
        );
        assert_eq!(after_move.opponent_hand, Hand::default().without(Diagonal));
        assert_eq!(after_move.unavailable, Board::from(0b_1111_1010_1111_1111));
    }
}
