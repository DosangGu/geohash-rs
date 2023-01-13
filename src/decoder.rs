//! Decoder for geohash-rs

use crate::base32;
use crate::LEFT;
use crate::RIGHT;
use std::collections::VecDeque;

/// For each 5-bit chunk of the geohash, we calculate the midpoint of the current bound and then split
/// the bound into two halves based on the value of the 5-bit chunk
///
/// Arguments:
///
/// * `geohash`: The geohash string to decode.
///
/// Returns:
///
/// A GPSBoundInfo struct
///
/// Panics:
///
/// Panics when geohash String is invalid
///
/// # Examples
///
/// ```
/// use geohash_rs;
/// let gps_bounds = geohash_rs::decode(String::from("wydm9qyc"));
///
/// assert_eq!(gps_bounds.latitude[0], 37.56654739379883);
/// assert_eq!(gps_bounds.latitude[1], 37.56671905517578);
/// assert_eq!(gps_bounds.longitude[0], 126.97826385498047);
/// assert_eq!(gps_bounds.longitude[1], 126.97860717773438);
/// ```
pub fn decode(geohash: String) -> GPSBoundInfo {
    let geo_bits: Vec<u8> = geohash.chars().map(|ch| base32::decode_b32(&ch)).collect();
    let mut lat_bound: [f32; 2] = [-90.0, 90.0];
    let mut long_bound: [f32; 2] = [-180.0, 180.0];
    let mut bound_queue = VecDeque::from([&mut long_bound, &mut lat_bound]);

    for bit_5 in geo_bits {
        cal_coord_bound(&bit_5, &mut bound_queue);
    }

    GPSBoundInfo {
        latitude: lat_bound,
        longitude: long_bound,
    }
}

/// `cal_coord_bound` takes a `bit_5` and a `bound_queue` and for each of the 5 bits in `bit_5` it calls
/// `cal_bound` with the `bit_5` and the bit.
///
/// Arguments:
///
/// * `bit_5`: the 5-bit binary representation of the current number
/// * `bound_queue`: a queue of bounding boxes, each bounding box is represented by a 2-element array,
/// the first element is the lower bound, the second element is the upper bound.
fn cal_coord_bound(bit_5: &u8, bound_queue: &mut VecDeque<&mut [f32; 2]>) {
    for i in [4_u8, 3, 2, 1, 0] {
        cal_bound(bit_5, &i, bound_queue);
    }
}

/// It takes a bit and a bound queue, and updates the bound queue
///
/// Arguments:
///
/// * `bit_5`: The 5-bit binary representation of the current character.
/// * `i`: the index of the bit we're currently looking at
/// * `bound_queue`: A queue of the bounds of the current bit.
fn cal_bound(bit_5: &u8, i: &u8, bound_queue: &mut VecDeque<&mut [f32; 2]>) {
    let bound_info = bound_queue.pop_front().unwrap();
    let mid = bound_info.iter().sum::<f32>() / 2.0;
    if 0 < (bit_5 & 1 << i) {
        bound_info[LEFT] = mid;
    } else {
        bound_info[RIGHT] = mid;
    }
    bound_queue.push_back(bound_info);
}

/// `GPSBoundInfo` is a struct that contains two arrays of two floats each, one for latitude and one for
/// longitude.
///
/// Properties:
///
/// * `latitude`: [f32;2] - The latitude of the bounding box. The first value is the minimum latitude,
/// the second value is the maximum latitude.
/// * `longitude`: The longitude of the location.
pub struct GPSBoundInfo {
    pub latitude: [f32; 2],
    pub longitude: [f32; 2],
}
