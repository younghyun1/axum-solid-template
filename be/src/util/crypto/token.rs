use sha2::{Digest, Sha256};

const OPAQUE_TOKEN_BYTES: usize = 32;
const HEX_ALPHABET: &[u8; 16] = b"0123456789abcdef";

pub fn generate_opaque_token() -> String {
    let mut bytes = [0_u8; OPAQUE_TOKEN_BYTES];
    rand::fill(&mut bytes);
    encode_hex(&bytes)
}

pub fn sha256_hex(value: &str) -> String {
    let digest = Sha256::digest(value.as_bytes());
    encode_hex(digest.as_ref())
}

fn encode_hex(bytes: &[u8]) -> String {
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        encoded.push(HEX_ALPHABET[(byte >> 4) as usize] as char);
        encoded.push(HEX_ALPHABET[(byte & 0x0f) as usize] as char);
    }
    encoded
}

#[cfg(test)]
mod tests {
    use super::{generate_opaque_token, sha256_hex};

    #[test]
    fn generated_tokens_are_hex_256_bit_values() {
        let token = generate_opaque_token();

        assert_eq!(token.len(), 64);
        assert!(token.chars().all(|character| character.is_ascii_hexdigit()));
    }

    #[test]
    fn sha256_hex_is_stable() {
        assert_eq!(
            sha256_hex("refresh-token"),
            "0eb17643d4e9261163783a420859c92c7d212fa9624106a12b510afbec266120"
        );
    }
}
