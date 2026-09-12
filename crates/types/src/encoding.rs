use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncodingError {
    LengthOverflow,
}

impl fmt::Display for EncodingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LengthOverflow => f.write_str("length does not fit canonical u32 encoding"),
        }
    }
}

impl std::error::Error for EncodingError {}

/// Canonical unsigned integer encoding used by the NOVA reference encoder.
/// Fixed-width integers are little-endian; variable byte strings are prefixed
/// by an unsigned 32-bit little-endian length.
pub fn put_u8(out: &mut Vec<u8>, value: u8) {
    out.push(value);
}

pub fn put_u32_le(out: &mut Vec<u8>, value: u32) {
    out.extend_from_slice(&value.to_le_bytes());
}

pub fn put_u64_le(out: &mut Vec<u8>, value: u64) {
    out.extend_from_slice(&value.to_le_bytes());
}

pub fn put_u128_le(out: &mut Vec<u8>, value: u128) {
    out.extend_from_slice(&value.to_le_bytes());
}

pub fn put_bytes(out: &mut Vec<u8>, value: &[u8]) -> Result<(), EncodingError> {
    let len = u32::try_from(value.len()).map_err(|_| EncodingError::LengthOverflow)?;
    put_u32_le(out, len);
    out.extend_from_slice(value);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_width_values_are_little_endian() {
        let mut out = Vec::new();
        put_u8(&mut out, 0x12);
        put_u32_le(&mut out, 0x01020304);
        put_u64_le(&mut out, 0x0102030405060708);
        put_u128_le(&mut out, 0x0102030405060708090a0b0c0d0e0f10);
        assert_eq!(
            out,
            vec![
                0x12, 0x04, 0x03, 0x02, 0x01,
                0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01,
                0x10, 0x0f, 0x0e, 0x0d, 0x0c, 0x0b, 0x0a, 0x09,
                0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01,
            ]
        );
    }

    #[test]
    fn byte_strings_are_length_prefixed() {
        let mut out = Vec::new();
        put_bytes(&mut out, b"NOVA").unwrap();
        assert_eq!(out, vec![4, 0, 0, 0, b'N', b'O', b'V', b'A']);
    }
}
