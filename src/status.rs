//! Defines an enum representing the victory status.

use std::ops::Not;

/// Represents the victory status.
#[derive(Debug, PartialEq, Eq)]
pub enum Status {
    Win,
    Draw,
    Loss,
}

impl Not for Status {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Status::Win => Status::Loss,
            Status::Draw => self,
            Status::Loss => Status::Win,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Status::*;

    #[test]
    fn not() {
        assert_eq!(Loss, !Win);
        assert_eq!(Draw, !Draw);
        assert_eq!(Win, !Loss);
    }
}
