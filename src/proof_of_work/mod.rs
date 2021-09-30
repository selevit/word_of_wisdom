pub mod proto;
pub use proto::PUZZLE_SIZE;
use proto::{Puzzle, PuzzleSolution};
use rand::Rng;
use serde::de::DeserializeOwned;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::io::{Read, Write};

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

        leading_zeros >= self.complexity
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

pub struct Transport<T: Read + Write> {
    c: T,
}

impl<T> Transport<T>
where
    T: Read + Write,
{
    pub fn new(c: T) -> Self {
        Self { c }
    }

    pub fn send<V>(&mut self, value: &V) -> anyhow::Result<()>
    where
        V: Serialize,
    {
        self.c.write_all(&bincode::serialize(value)?)?;
        Ok(())
    }

    pub fn send_with_varsize<V>(&mut self, value: &V) -> anyhow::Result<()>
    where
        V: Serialize,
    {
        let data = bincode::serialize(value)?;
        let len = bincode::serialize(&data.len())?;
        self.c.write_all(&len)?;
        self.c.write_all(&data)?;
        Ok(())
    }

    pub fn receive<R>(&mut self, size: usize) -> anyhow::Result<R>
    where
        R: DeserializeOwned,
    {
        let mut buf: Vec<u8> = vec![0; size];
        self.c.read_exact(&mut buf)?;
        let result: R = bincode::deserialize(&buf)?;
        Ok(result)
    }
}
