use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Puzzle {
    pub complexity: u8,
    pub value: [u8; 16],
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct PuzzleSolution([u8; 16]);

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum SolutionState {
    ACCEPTED,
    REJECTED,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct QuoteSize(usize);

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct WordOfWisdomQuote(String);

impl PuzzleSolution {
    pub fn new(s: [u8; 16]) -> Self {
        Self(s)
    }

    pub fn as_bytes(&self) -> [u8; 16] {
        self.0
    }
}

impl QuoteSize {
    pub fn new(s: usize) -> Self {
        Self(s)
    }

    pub fn size(&self) -> usize {
        self.0
    }
}

impl WordOfWisdomQuote {
    pub fn new(s: &str) -> Self {
        Self(String::from(s))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
