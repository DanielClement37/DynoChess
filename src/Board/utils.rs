use super::{defs::Location, Board};
use crate::{
    board::defs::Ranks,
    defs::{Side, Sides, Square},
};

impl Board{
    // given a square find the file and rank
    pub fn square_location(square:Square)->Location{
        let file = (square % 8) as u8; // square mod 8
        let rank = (square / 8) as u8; // square div 8
        return (file, rank);
    }

    // check if square is or isn't on the given rank
    pub fn is_square_on_rank(square: Square, rank: Square)-> bool{
        let start = (rank) * 8;
        let end = start + 7;
        return (start..=end).contains(&square)
    }

    // return the fourth rank for given side
    pub fn fourth_rank(side:Side)->usize{
        return if side == Sides::WHITE {
            Ranks::R4
        } else {
            Ranks::R5
        }
    }

    // return the promotion rank for given side
    pub fn promotion_rank(side:Side)->usize{
        return if side == Sides::WHITE {
            Ranks::R8
        } else {
            Ranks::R1
        }
    }
}