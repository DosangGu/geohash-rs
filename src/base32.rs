const BASE32_ENCODING: [char; 32] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'b', 'c', 'd', 'e', 'f', 'g',
    'h', 'j', 'k', 'm', 'n', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
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

/// get base32 encoded char from u8
pub fn encode_b32(i: &u8) -> char {
    return BASE32_ENCODING[*i as usize];
}
/// get base32 decoded u8 from char
pub fn decode_b32(c: &char) -> u8 {
    if !BASE32_ENCODING.contains(c) {
        panic!("invalid char")
    } else {
        return BASE32_DECODING[*c as usize];
    }
}



// pub fn encode_b32(i: &u8) -> char {
//     if i == &(0u8) {
//         return '0';
//     } else if i == &(1) {
//         return '1';
//     } else if i == &(2) {
//         return '2';
//     } else if i == &(3) {
//         return '3';
//     } else if i == &(4) {
//         return '4';
//     } else if i == &(5) {
//         return '5';
//     } else if i == &(6) {
//         return '6';
//     } else if i == &(7) {
//         return '7';
//     } else if i == &(8) {
//         return '8';
//     } else if i == &(9) {
//         return '9';
//     } else if i == &(10) {
//         return 'b';
//     } else if i == &(11) {
//         return 'c';
//     } else if i == &(12) {
//         return 'd';
//     } else if i == &(13) {
//         return 'e';
//     } else if i == &(14) {
//         return 'f';
//     } else if i == &(15) {
//         return 'g';
//     } else if i == &(16) {
//         return 'h';
//     } else if i == &(17) {
//         return 'j';
//     } else if i == &(18) {
//         return 'k';
//     } else if i == &(19) {
//         return 'm';
//     } else if i == &(20) {
//         return 'n';
//     } else if i == &(21) {
//         return 'p';
//     } else if i == &(22) {
//         return 'q';
//     } else if i == &(23) {
//         return 'r';
//     } else if i == &(24) {
//         return 's';
//     } else if i == &(25) {
//         return 't';
//     } else if i == &(26) {
//         return 'u';
//     } else if i == &(27) {
//         return 'v';
//     } else if i == &(28) {
//         return 'w';
//     } else if i == &(29) {
//         return 'x';
//     } else if i == &(30) {
//         return 'y';
//     } else if i == &(31) {
//         return 'z';
//     } else {
//         return 'a';
//     }
// }