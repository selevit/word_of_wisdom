pub mod proto;
use proto::{Puzzle, PuzzleSolution, SolutionState};
use rand::Rng;
use sha2::{Digest, Sha256};

impl Puzzle {
    pub fn verify(&self, solution: &PuzzleSolution) -> SolutionState {
        let mut hasher = Sha256::new();
        hasher.update(self.value);
        hasher.update(solution.as_bytes());

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

        if leading_zeros >= self.complexity {
            SolutionState::ACCEPTED
        } else {
            SolutionState::REJECTED
        }
    }

    pub fn solve(&self) -> PuzzleSolution {
        loop {
            let solution = rand::thread_rng().gen::<[u8; 16]>();
            let solution = PuzzleSolution::new(solution);
            if self.verify(&solution) == SolutionState::ACCEPTED {
                return solution;
            }
        }
    }
}
