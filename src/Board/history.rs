use super::gamestate::GameState;
use crate::defs::MAX_GAME_MOVES;

// The history struct is basically an array holding the values of the game
// states at each move. If a move is made in make(), this function pushes the
// current game state into this array. In unmake(), that game state can then be
// popped and restored. It is faster than a vector, because:
//
// - It is stored on the stack (a vector is stored on the heap)
// - It doesn't do any error checking. It is up to the caller to check if the
//   history array is either full or empty, before pushing or popping (if
//   necessary, such as during console play: the chess engine will always have
//   one push for every pop during search.)

#[derive(Clone)]
pub struct History {
    list: [GameState; MAX_GAME_MOVES as usize],
    count: usize,
}

impl History{
    pub fn new()->Self{
        Self{
            list: [GameState::new(); MAX_GAME_MOVES as usize],
            count: 0,
        }
    }

    //Clear fn that wipes the array
    pub fn clear(&mut self) {
        self.list = [GameState::new(); MAX_GAME_MOVES as usize];
        self.count = 0;
    }

    //push fn to put a new gamestate into the array
    pub fn put(&mut self, gs:GameState){
        self.list[self.count] = gs;
        self.count += 1;
    }

    //pop fn to return and remove the last gamestate added to the array
    pub fn pop(&mut self) ->GameState{
        self.count -= 1;
        self.list[self.count]
    }


    //get reference. returns a pointer to the gamestate
    pub fn get_ref(&self, index: usize) -> &GameState {
        &self.list[index]
    }

    //len. returns the array length
    pub fn len(&self) -> usize {
        self.count
    }
}