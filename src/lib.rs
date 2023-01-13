//! 'geohash_rs' is a geohash encoder for rust.

pub mod base32;
pub mod decoder;
pub mod encoder;
const LEFT: usize = 0;
const RIGHT: usize = 1;

pub use decoder::decode;
pub use encoder::encode;
