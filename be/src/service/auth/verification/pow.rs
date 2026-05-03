use sha2::{Digest, Sha256};
use uuid::Uuid;

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

    #[test]
    fn leading_zero_bits_accepts_matching_prefix() {
        assert!(has_leading_zero_bits(&[0b0000_1111], 4));
        assert!(!has_leading_zero_bits(&[0b0001_1111], 4));
        assert!(has_leading_zero_bits(&[0, 0b0011_1111], 10));
        assert!(!has_leading_zero_bits(&[0, 0b0111_1111], 10));
    }
}
