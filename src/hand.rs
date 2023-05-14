//! Defines a struct representing a player's hand.

use crate::tile::Tile;

/// Represents a player's hand.
///
/// The hand is represented as an array of four `u8`s, which relies on the
/// `Tile` enumeration having discriminants 0 to 3.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Hand([u8; 4]);

impl Default for Hand {
    /// Returns the default hand containing two of each tile.
    fn default() -> Self {
        Hand([2, 2, 2, 2])
    }
}

impl Hand {
    /// Returns whether the hand is empty.
    pub fn is_empty(&self) -> bool {
        (self.0[0] + self.0[1] + self.0[2] + self.0[3]) == 0
    }

    /// Returns whether the hand contains the specified tile.
    pub fn has(&self, tile: Tile) -> bool {
        self.0[tile as usize] > 0
    }

    /// Returns a new hand with one of the specified tile having been removed.
    pub fn without(&self, tile: Tile) -> Self {
        let mut hand = *self;
        hand.0[tile as usize] -= 1;
        hand
    }
}

#[cfg(test)]
mod tests {
    use super::Hand;
    use crate::tile::Tile::*;

    #[test]
    fn is_empty() {
        assert!(Hand([0, 0, 0, 0]).is_empty());
        assert!(!Hand([1, 0, 0, 0]).is_empty());
        assert!(!Hand([0, 1, 0, 0]).is_empty());
        assert!(!Hand([0, 0, 1, 0]).is_empty());
        assert!(!Hand([0, 0, 0, 1]).is_empty());
    }

    #[test]
    fn has() {
        assert_has(Hand([1, 0, 0, 0]), true, false, false, false);
        assert_has(Hand([0, 1, 0, 0]), false, true, false, false);
        assert_has(Hand([0, 0, 1, 0]), false, false, true, false);
        assert_has(Hand([0, 0, 0, 1]), false, false, false, true);
    }

    #[test]
    fn without() {
        let hand = Hand([1, 1, 1, 1]);
        assert_has(hand.without(Puller), false, true, true, true);
        assert_has(hand.without(Pusher), true, false, true, true);
        assert_has(hand.without(Straight), true, true, false, true);
        assert_has(hand.without(Diagonal), true, true, true, false);
    }

    fn assert_has(hand: Hand, puller: bool, pusher: bool, straight: bool, diagonal: bool) {
        assert_eq!(hand.has(Puller), puller);
        assert_eq!(hand.has(Pusher), pusher);
        assert_eq!(hand.has(Straight), straight);
        assert_eq!(hand.has(Diagonal), diagonal);
    }
}
