use sha2::{Digest, Sha256};

pub fn verify_challenge(complexity: u8, challenge: &[u8], solution: &[u8]) -> bool {
    let mut hasher = Sha256::new();
    hasher.update(challenge);
    hasher.update(solution);
    let result = hasher.finalize();
    let mut leading_zeros = 0;
    for c in result.iter().take(complexity as usize / 2 + 1) {
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
    leading_zeros >= complexity
}
