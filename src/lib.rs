//! 'geohash_rs' is a geohash encoder for rust.

use std::collections::VecDeque;

pub mod base32;

const LEFT: usize = 0;
const RIGHT: usize = 1;

/// Encodes gps coordinates to String
/// 
/// # Panics
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
    if !(-90.0<=latitude && latitude <= 90.0) || !(-180.0<=longitude && longitude<=180.0) {
        panic!("input coordinates out of bound");
    }
    let mut lat_info = CoordInfo {
        bound: [-90.0,90.0],
        coord: latitude
    };
    let mut long_info = CoordInfo {
        bound: [-180.0,180.0],
        coord: longitude
    };
    let mut coord_queue = VecDeque::from([&mut long_info, &mut lat_info]);
    let geohash_string: String = (0..length_geohash).into_iter()
                                                    .map(|_| get_geo_bit(&mut coord_queue))
                                                    .map(|a| base32::encode_b32(&a))
                                                    .collect();
    return geohash_string;
}
/// calculate 5 digit geo-bit from coordinate
/// return u8
fn get_geo_bit(coord_queue: &mut VecDeque<&mut CoordInfo>) -> u8 {
    return [4 as u8,3,2,1,0].iter()
                            .map(|i| cal_bit(coord_queue, *i))
                            .reduce(|x, y| x | y)
                            .unwrap();
}
/// calculate 1 digit geo-bit from coordinate
/// returns u8
fn cal_bit(coord_queue: &mut VecDeque<&mut CoordInfo>, i: u8) -> u8 {
    // const LEFT: usize = 0;
    // const RIGHT: usize = 1;
    let mut coord_info_inst = coord_queue.pop_front().unwrap();
    let mid = &(coord_info_inst.bound).iter().sum::<f32>() / 2.0;
    if &mid <= &(coord_info_inst.coord) {
        coord_info_inst.bound = [mid, coord_info_inst.bound[RIGHT]];
        coord_queue.push_back(coord_info_inst);
        return 1 << i;
    } else {
        coord_info_inst.bound = [coord_info_inst.bound[LEFT], mid];
        coord_queue.push_back(coord_info_inst);
        return 0;
    }
}
struct CoordInfo {
    bound: [f32;2],
    coord: f32
}

/// Decodes geohash String to gps coordinate bounds
/// 
/// # Panics
/// Panics when geohash String is invalid
/// 
/// # Examples
/// 
/// ```
/// use geohash_rs::{self, GPSBoundInfo};
/// let gps_bounds: GPSBoundInfo = geohash_rs::decode(String::from("wydm9qyc"));
/// 
/// assert_eq!(gps_bounds.latitude[0], 37.56654739379883);
/// assert_eq!(gps_bounds.latitude[1], 37.56671905517578);
/// assert_eq!(gps_bounds.longitude[0], 126.97826385498047);
/// assert_eq!(gps_bounds.longitude[1], 126.97860717773438);
/// ```
pub fn decode(geohash: String) -> GPSBoundInfo {
    let geo_bits: Vec<u8> = geohash.chars()
                                   .map(|ch| base32::decode_b32(&ch))
                                   .collect();
    let mut lat_bound: [f32;2] = [-90.0, 90.0];
    let mut long_bound: [f32;2] = [-180.0, 180.0];
    let mut bound_queue = VecDeque::from([&mut long_bound, &mut lat_bound]);

    for bit_5 in geo_bits {
        cal_coord_bound(&bit_5, &mut bound_queue);
    }

    return GPSBoundInfo { latitude: lat_bound, longitude: long_bound };
}

fn cal_coord_bound(bit_5: &u8, bound_queue: &mut VecDeque<&mut [f32;2]>) {
    for i in [4 as u8, 3, 2, 1, 0] {
        cal_bound(bit_5, &i, bound_queue)
    }
}

fn cal_bound(bit_5: &u8, i: &u8, bound_queue: &mut VecDeque<&mut [f32;2]>) {
    let bound_info = bound_queue.pop_front().unwrap();
    let mid = bound_info.iter().sum::<f32>() / 2.0;
    if 0 < (bit_5 & 1 << i) {
        bound_info[LEFT] = mid;
    } else {
        bound_info[RIGHT] = mid;
    }
    bound_queue.push_back(bound_info);
}
pub struct GPSBoundInfo {
    pub latitude: [f32;2],
    pub longitude: [f32;2]
}