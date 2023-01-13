//! Encoder for geohash-rs

use crate::base32;
use crate::LEFT;
use crate::RIGHT;
use std::collections::VecDeque;

/// We take the latitude and longitude, and we split the geohash into two parts, one for the latitude
/// and one for the longitude. We then iterate over the length of the geohash, and for each iteration we
/// get the next bit of the geohash, and then we encode that bit into base32
///
///
/// Arguments:
///
/// * `latitude`: the latitude of the point you want to encode
/// * `longitude`: the longitude of the point you want to encode
/// * `length_geohash`: The length of the geohash string. The longer the string, the more precise the
/// geohash.
///
/// Returns:
///
/// A string of length length_geohash, which is the geohash of the input coordinates.
///
/// Panics:
///
/// Panics when latitude input does not bound into -90.0 ~ 90.0 or longitude input does not bound into -180 ~ 180
///
/// # Examples
///
/// ```
/// use geohash_rs;
/// let geohash = geohash_rs::encode(37.5666805, 126.9784147, 8);
///
/// assert_eq!(geohash, String::from("wydm9qyc"))
/// ```
pub fn encode(latitude: f32, longitude: f32, length_geohash: u32) -> String {
    if !(-90.0..=90.0).contains(&latitude) || !(-180.0..=180.0).contains(&longitude) {
        panic!("input coordinates out of bound");
    }
    let mut lat_info = CoordInfo {
        bound: [-90.0, 90.0],
        coord: latitude,
    };
    let mut long_info = CoordInfo {
        bound: [-180.0, 180.0],
        coord: longitude,
    };
    let mut coord_queue = VecDeque::from([&mut long_info, &mut lat_info]);
    let geohash_string: String = (0..length_geohash)
        .into_iter()
        .map(|_| get_geo_bit(&mut coord_queue))
        .map(|a| base32::encode_b32(&a))
        .collect();
    geohash_string
}

/// It calculates the geo bit for a given coordinate.
///
/// Arguments:
///
/// * `coord_queue`: a queue of CoordInfo structs, which are the coordinates of the current pixel and
/// its neighbors.
///
/// Returns:
///
/// A bitmask of the geo index of the current coordinate.
fn get_geo_bit(coord_queue: &mut VecDeque<&mut CoordInfo>) -> u8 {
    return [4_u8, 3, 2, 1, 0]
        .iter()
        .map(|i| cal_bit(coord_queue, *i))
        .reduce(|x, y| x | y)
        .unwrap();
}

/// It takes a mutable reference to a vector of mutable references to CoordInfo structs, and returns a
/// u8
///
/// Arguments:
///
/// * `coord_queue`: a queue of CoordInfo structs, which contain the coordinates and bounds of the
/// current coordinate
/// * `i`: the current bit we're calculating
///
/// Returns:
///
/// A bit is being returned.
fn cal_bit(coord_queue: &mut VecDeque<&mut CoordInfo>, i: u8) -> u8 {
    let mut coord_info_inst = coord_queue.pop_front().unwrap();
    let mid = &(coord_info_inst.bound).iter().sum::<f32>() / 2.0;
    if mid <= coord_info_inst.coord {
        coord_info_inst.bound[LEFT] = mid;
        coord_queue.push_back(coord_info_inst);
        1 << i
    } else {
        coord_info_inst.bound[RIGHT] = mid;
        coord_queue.push_back(coord_info_inst);
        0
    }
}

struct CoordInfo {
    bound: [f32; 2],
    coord: f32,
}
