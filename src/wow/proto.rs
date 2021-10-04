use serde::{Deserialize, Serialize};

pub const PUZZLE_SIZE: usize = 16;
pub const SOLUTION_SIZE: usize = 16;
pub const SOLUTION_STATE_SIZE: usize = 4;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Puzzle {
    pub complexity: u8,
    pub value: [u8; PUZZLE_SIZE],
}

pub type PuzzleSolution = [u8; SOLUTION_SIZE];

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum SolutionState {
    Accepted,
    Rejected,
}
