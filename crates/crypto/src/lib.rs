use nova_types::Hash;

pub fn hash_bytes(data: &[u8]) -> Hash {
    *blake3::hash(data).as_bytes()
}

#[cfg(test)]
mod tests {
    use super::hash_bytes;

    #[test]
    fn hashing_is_deterministic() {
        assert_eq!(hash_bytes(b"nova"), hash_bytes(b"nova"));
    }
}
