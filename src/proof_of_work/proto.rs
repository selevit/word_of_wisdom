use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Puzzle {
    pub complexity: u8,
    pub value: [u8; 16],
}

impl Puzzle {
    pub fn new(complexity: u8) -> Self {
        let value = rand::thread_rng().gen::<[u8; 16]>();
        Puzzle { complexity, value }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct PuzzleSolution([u8; 16]);

impl PuzzleSolution {
    pub fn new(s: [u8; 16]) -> Self {
        Self(s)
    }

    pub fn as_bytes(&self) -> [u8; 16] {
        self.0
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum SolutionResponse {
    ACCEPTED,
    REJECTED,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct QuoteSize(usize);

impl QuoteSize {
    pub fn new(s: usize) -> Self {
        Self(s)
    }

    pub fn size(&self) -> usize {
        self.0
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct WordOfWisdomQuote(String);

impl WordOfWisdomQuote {
    pub fn new(s: &str) -> Self {
        Self(String::from(s))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
