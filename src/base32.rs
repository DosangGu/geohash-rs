//! BASE32 encoder and decoder for geohash-rs

const BASE32_ENCODING: [char; 32] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'j', 'k',
    'm', 'n', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
];
const BASE32_DECODING: [u8; 128] = [
    b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0',
    b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0',
    b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0',
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, b'0', b'0', b'0', b'0', b'0', b'0',
    b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0',
    b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0',
    b'0', b'0', 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, b'0', 0x11, 0x12, b'0', 0x13, 0x14, b'0',
    0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, b'0', b'0', b'0', b'0', b'0',
];

/// It takes a byte and returns the base32 character that corresponds to that byte
///
/// Arguments:
///
/// * `i`: The byte to encode.
///
/// Returns:
///
/// A character
pub fn encode_b32(i: &u8) -> char {
    BASE32_ENCODING[*i as usize]
}

/// It takes a character and returns the corresponding value in the decoding table
///
/// Arguments:
///
/// * `c`: the character to decode
///
/// Returns:
///
/// A u8
pub fn decode_b32(c: &char) -> u8 {
    if !BASE32_ENCODING.contains(c) {
        panic!("invalid char")
    } else {
        BASE32_DECODING[*c as usize]
    }
}
