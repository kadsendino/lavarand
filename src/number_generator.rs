use num_bigint::BigUint;
use num_traits::{One, ToPrimitive};
use std::str::FromStr;

/// Step 4: Seed a Blum Blum Shub PRNG using a byte slice as seed
///
/// # Arguments
/// * `seed_bytes` - The 140-byte chaotic seed from SHA-1 7-way hashing
/// * `num_bytes` - Number of output random bytes desired
///
/// # Returns
/// A Vec<u8> containing the pseudo-random bytes
pub fn blum_blum_shub(seed_bytes: &[u8], num_bytes: usize) -> Vec<u8> {
    // 1062-bit modulus (product of two secret Blum primes)
    let m = BigUint::from_str(
        "53607511146220112360879790227353067135925376599474552853531481818526455500785383194936281698585494806464721601253386562653528822554332256253291788639699229111681587721849555921168837796146614508574462019262321980150281379240551466508660971979094060899184914133568666470293429076893274969016410915119621659901793350665953"
    ).unwrap();

    // Convert the 140-byte seed into a BigUint
    let seed = BigUint::from_bytes_be(seed_bytes);
    let mut x = seed % &m;

    let mut output = Vec::with_capacity(num_bytes);

    // Generate num_bytes pseudo-random bytes
    for _ in 0..num_bytes {
        // Square modulo M
        x = (&x * &x) % &m;

        // Take the low 8 bits as the random byte
        let byte = (&x & BigUint::from(0xffu8)).to_u8().unwrap();
        output.push(byte);
    }

    output
}

