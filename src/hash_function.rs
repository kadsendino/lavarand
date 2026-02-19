use sha1::{Sha1, Digest};
use sha2::{Sha256};

// This function takes a byte slice and an integer n, and produces a new byte vector that contains
// n SHA-1 hashes.
pub fn n_way_sha1(bytes: &[u8], n: usize) -> Vec<u8> {
    let mut result = Vec::with_capacity(n * 20);

    for offset in 0..n {
        let mut hasher = Sha1::new();

        for chunk in bytes.iter().skip(offset).step_by(n) {
            hasher.update(&[*chunk]);
        }

        let hash = hasher.finalize();
        result.extend_from_slice(&hash);
    }

    result
}

// Similar to n_way_sha1 but using SHA-256 which produces 32-byte hashes
pub fn n_way_sha256(bytes: &[u8], n: usize) -> Vec<u8> {
    let mut result = Vec::with_capacity(n * 32);

    for offset in 0..n {
        let mut hasher = Sha256::new();

        for byte in bytes.iter().skip(offset).step_by(n) {
            hasher.update(&[*byte]);
        }

        let hash = hasher.finalize();
        result.extend_from_slice(&hash);
    }

    result
}

