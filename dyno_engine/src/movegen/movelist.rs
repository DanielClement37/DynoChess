#![allow(dead_code)]
#![allow(unused_variables)]

use crate::defs::{MAX_LEGAL_MOVES};
use std::mem::MaybeUninit;
use crate::movegen::defs::Move;
use serde::{Serialize, Deserialize};
use serde_big_array::BigArray;

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub struct MoveList{
    #[serde(with = "BigArray")]
    list: [Move; MAX_LEGAL_MOVES as usize],
    count: u8
}

impl MoveList {
    // Creates a new move list. Memory is not initialized. This is not
    // necessary, because the next step will always be to generate moves, and
    // store them in the list beginning at index 0, which would overwrite the
    // initialization. Therefore, doing memory initialization slows the program
    // down. (Tests show this slowdown to be around 50%-60%.)
    pub fn new() -> Self {
        Self {
            list: unsafe {
                #[allow(clippy::uninit_assumed_init)]
                MaybeUninit::assume_init(MaybeUninit::uninit())
            },
            count: 0,
        }
    }

    // Used to store a move in the move list.
    pub fn push(&mut self, m: Move) {
        self.list[self.count as usize] = m;
        self.count += 1;
    }

    // Returns the number of moves in the move list.
    pub fn len(&self) -> u8 {
        self.count
    }

    // Return the move at the given index. If out of bounds, the program crashes.
    pub fn get_move(&self, index: u8) -> Move {
        self.list[index as usize]
    }

    pub fn get_mut_move(&mut self, index: u8) -> &mut Move {
        &mut self.list[index as usize]
    }

    pub fn swap(&mut self, a: usize, b: usize) {
        unsafe {
            // Take two raw pointers to the moves.
            let ptr_a: *mut Move = &mut self.list[a];
            let ptr_b: *mut Move = &mut self.list[b];

            // Swap those pointers, so now the moves are swapped.
            std::ptr::swap(ptr_a, ptr_b);
        }
    }
}