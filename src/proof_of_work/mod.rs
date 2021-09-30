pub mod proto;
use proto::{Puzzle, PuzzleSolution};
use sha2::{Digest, Sha256};

pub fn verify_challenge(puzzle: &Puzzle, solution: &PuzzleSolution) -> bool {
    let mut hasher = Sha256::new();
    hasher.update(puzzle.value);
    hasher.update(solution.as_bytes());

    let result = hasher.finalize();
    let mut leading_zeros = 0;
    for c in result.iter().take(puzzle.complexity as usize / 2 + 1) {
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
    leading_zeros >= puzzle.complexity
}
