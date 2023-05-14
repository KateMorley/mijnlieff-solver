//! Defines a struct representing a board.

use std::fmt::{self, Display};

/// Represents a board.
///
/// The board is represented as a `u16` bit field with the squares corresponding
/// to these bits:
///
/// ```
///  0  1  2  3
///  4  5  6  7
///  8  9 10 11
/// 12 13 14 15
/// ```
///
/// The `From` trait is implemented to allow conversion to and from a `u16`.
///
/// As only one bit of information is stored per square (with 0 representing an
/// available square and 1 representing an occupied square), the squares
/// occupied by the two players must be represented as separate instances.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct Board(u16);

impl From<Board> for u16 {
    fn from(value: Board) -> u16 {
        value.0
    }
}

impl From<u16> for Board {
    fn from(value: u16) -> Board {
        Board(value)
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();

        for square in 0..=15 {
            if square > 0 {
                s.push(if square % 4 == 0 { '\n' } else { ' ' });
            }

            s.push(if self.is_available(square) {
                '□'
            } else {
                '■'
            });
        }

        f.write_str(&s)
    }
}

impl Board {
    /// Bit fields representing the possible lines of three squares.
    ///
    /// Only used in evaluating the SCORES constant at compile time.
    const LINES: [u16; 24] = [
        0b_0000_0000_0000_0111,
        0b_0000_0000_0000_1110,
        0b_0000_0000_0111_0000,
        0b_0000_0000_1110_0000,
        0b_0000_0111_0000_0000,
        0b_0000_1110_0000_0000,
        0b_0111_0000_0000_0000,
        0b_1110_0000_0000_0000,
        0b_0000_0001_0001_0001,
        0b_0000_0010_0010_0010,
        0b_0000_0100_0100_0100,
        0b_0000_1000_1000_1000,
        0b_0001_0001_0001_0000,
        0b_0010_0010_0010_0000,
        0b_0100_0100_0100_0000,
        0b_1000_1000_1000_0000,
        0b_0000_0100_0010_0001,
        0b_0000_1000_0100_0010,
        0b_0100_0010_0001_0000,
        0b_1000_0100_0010_0000,
        0b_0000_0001_0010_0100,
        0b_0000_0010_0100_1000,
        0b_0001_0010_0100_0000,
        0b_0010_0100_1000_0000,
    ];

    /// An array whose `i`th element is the score for the board whose `u16`
    /// value is `i`.
    const SCORES: [u8; 65536] = Self::calculate_scores();

    /// Returns the scores for all possible boards.
    ///
    /// Only used in evaluating the SCORES constant at compile time.
    const fn calculate_scores() -> [u8; 65536] {
        let mut scores = [0; 65536];

        let mut board = 0;

        loop {
            let mut line = 0;

            while line < 24 {
                if board & Self::LINES[line] == Self::LINES[line] {
                    scores[board as usize] += 1;
                }

                line += 1;
            }

            if board == u16::MAX {
                break;
            }

            board += 1;
        }

        scores
    }

    /// Returns the score for the board.
    pub fn get_score(&self) -> u8 {
        Self::SCORES[self.0 as usize]
    }

    /// Returns whether all squares are occupied.
    pub fn is_full(&self) -> bool {
        self.0 == 0b_1111_1111_1111_1111
    }

    /// Returns whether the specified square is available.
    pub fn is_available(&self, square: u8) -> bool {
        self.0 & (1 << square) == 0
    }

    /// Returns an instance with the specified square marked as occupied.
    pub fn with(&self, square: u8) -> Board {
        Board(self.0 | (1 << square))
    }

    /// Returns an instance with each square marked as occupied if it is ocupied
    /// in either of the specified boards.
    pub fn merge(board_1: Board, board_2: Board) -> Board {
        Board(board_1.0 | board_2.0)
    }

    /// Returns an instance with each square marked as occupied if it is ocupied
    /// in any of the specified boards.
    pub fn merge_3(board_1: Board, board_2: Board, board_3: Board) -> Board {
        Board(board_1.0 | board_2.0 | board_3.0)
    }
}

#[cfg(test)]
mod tests {
    use super::Board;

    #[test]
    fn from() {
        let bitfield = 0b_1000_0100_0010_0001;
        assert_eq!(bitfield, u16::from(Board::from(bitfield)));
    }

    #[test]
    fn fmt() {
        assert_eq!(
            "■ □ □ □\n□ ■ ■ □\n□ ■ ■ □\n■ ■ ■ ■",
            format!("{}", Board::from(0b_1111_0110_0110_0001))
        );
    }

    #[test]
    fn get_score() {
        assert_eq!(0, Board::from(0b_0000_0000_0000_0000).get_score());
        assert_eq!(1, Board::from(0b_0000_0000_0000_0111).get_score());
        assert_eq!(0, Board::from(0b_0110_1001_1001_0110).get_score());
        assert_eq!(6, Board::from(0b_0100_1110_0111_0010).get_score());
    }

    #[test]
    fn is_full() {
        assert!(!Board::from(0b_0111_1111_1111_1111).is_full());
        assert!(Board::from(0b_1111_1111_1111_1111).is_full());
    }

    #[test]
    fn is_available() {
        assert!(Board::from(0b_0000_0000_000_0000).is_available(0));
        assert!(!Board::from(0b_0000_0000_000_0001).is_available(0));
    }

    #[test]
    fn with() {
        assert_eq!(
            0b_1000_0000_0000_0010,
            u16::from(Board::default().with(1).with(15))
        )
    }

    #[test]
    fn merge() {
        assert_eq!(
            0b_0000_1111_1111_1111,
            u16::from(Board::merge(
                Board::from(0b_0000_0000_1111_1111),
                Board::from(0b_0000_1111_0000_1111)
            ))
        );
    }

    #[test]
    fn merge_3() {
        assert_eq!(
            0b_0011_1111_1111_1111,
            u16::from(Board::merge_3(
                Board::from(0b_0000_0000_1111_1111),
                Board::from(0b_0000_1111_0000_1111),
                Board::from(0b_0011_0011_0011_0011)
            ))
        );
    }
}
