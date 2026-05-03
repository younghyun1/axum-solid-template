use sha2::{Digest, Sha256};
use uuid::Uuid;

/// Verify the proof-of-work nonce for a challenge payload.
///
/// Builds a hash input from `challenge_id`, `salt`, and `nonce`, then checks it satisfies
/// the requested number of leading zero bits.
///
/// # Arguments
/// * `challenge_id` - Challenge identifier used in PoW material.
/// * `salt` - Randomized server-issued salt value.
/// * `nonce` - Client-supplied PoW nonce string.
/// * `difficulty_bits` - Required leading-zero-bit difficulty (0..=256).
///
/// # Returns
/// `true` if the PoW candidate is valid and respects all guards; otherwise `false`.
pub fn verify_pow(challenge_id: Uuid, salt: &str, nonce: &str, difficulty_bits: i32) -> bool {
    if nonce.trim().is_empty() || nonce.len() > 128 {
        return false;
    }
    if !(0..=256).contains(&difficulty_bits) {
        return false;
    }
    let input = format!("{challenge_id}:{salt}:{nonce}");
    let digest = Sha256::digest(input.as_bytes());
    has_leading_zero_bits(digest.as_slice(), difficulty_bits as u32)
}

/// Check whether a byte slice starts with the requested number of zero bits.
///
/// # Arguments
/// * `bytes` - Byte digest to evaluate.
/// * `difficulty_bits` - Number of required leading zero bits.
///
/// # Returns
/// `true` when the digest satisfies the bit prefix requirement.
fn has_leading_zero_bits(bytes: &[u8], difficulty_bits: u32) -> bool {
    let mut remaining_bits = difficulty_bits;
    for byte in bytes {
        if remaining_bits == 0 {
            return true;
        }
        if remaining_bits >= 8 {
            if *byte != 0 {
                return false;
            }
            remaining_bits -= 8;
            continue;
        }

        let mask = 0xff_u8 << (8 - remaining_bits);
        return (*byte & mask) == 0;
    }

    remaining_bits == 0
}

#[cfg(test)]
mod tests {
    use super::has_leading_zero_bits;

    /// Assert that matching prefixes are accepted and mismatches are rejected.
    #[test]
    fn leading_zero_bits_accepts_matching_prefix() {
        assert!(has_leading_zero_bits(&[0b0000_1111], 4));
        assert!(!has_leading_zero_bits(&[0b0001_1111], 4));
        assert!(has_leading_zero_bits(&[0, 0b0011_1111], 10));
        assert!(!has_leading_zero_bits(&[0, 0b0111_1111], 10));
    }
}
