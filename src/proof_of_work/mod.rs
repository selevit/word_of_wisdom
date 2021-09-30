pub mod proto;
pub use proto::PUZZLE_SIZE;
use proto::{Puzzle, PuzzleSolution};
use rand::Rng;
use sha2::{Digest, Sha256};

pub const DEFAULT_COMPLEXITY: u8 = 3;

impl Default for Puzzle {
    fn default() -> Self {
        Self::new(DEFAULT_COMPLEXITY)
    }
}

pub struct SolvingResult {
    pub solution: PuzzleSolution,
    pub hashes_tried: u128,
}

impl Puzzle {
    pub fn new(complexity: u8) -> Self {
        let value = rand::thread_rng().gen::<[u8; PUZZLE_SIZE]>();
        Puzzle { complexity, value }
    }

    pub fn is_valid_solution(&self, solution: &PuzzleSolution) -> bool {
        let mut hasher = Sha256::new();
        hasher.update(self.value);
        hasher.update(solution);

        let result = hasher.finalize();
        let mut leading_zeros = 0;
        for c in result.iter().take(self.complexity as usize / 2 + 1) {
            if c >> 4 == 0 {
                leading_zeros += 1;
            } else {
                break;
            }
            if c & 0xF == 0 {
                leading_zeros += 1;
            } else {
                break;
            }
        }
        println!("hash: {:x}, leading zeros: {:?}", result, leading_zeros);

        return leading_zeros >= self.complexity;
    }

    pub fn solve(&self) -> SolvingResult {
        let mut hashes_tried: u128 = 0;
        loop {
            let solution = rand::thread_rng().gen::<PuzzleSolution>();
            hashes_tried += 1;
            if self.is_valid_solution(&solution) {
                return SolvingResult {
                    solution,
                    hashes_tried,
                };
            }
        }
    }
}
