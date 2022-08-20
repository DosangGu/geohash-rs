//! 'geohash_rs' is a geohash encoder for rust.

use std::collections::VecDeque;
mod base32;
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
    let lat_info = CoordInfo {
        bound: [-90.0,90.0],
        coord: latitude
    };
    let long_info = CoordInfo {
        bound: [-180.0,180.0],
        coord: longitude
    };
    let mut coord_queue = VecDeque::from([long_info, lat_info]);
    let geohash_string: String = (0..length_geohash).into_iter()
                                                    .map(|_| get_geo_bit(&mut coord_queue))
                                                    .map(|a| base32::encode_b32(&a))
                                                    .collect();
    return geohash_string;
}
/// calculate 5 digit geo-bit from coordinate
/// return u8
fn get_geo_bit(coord_queue: &mut VecDeque<CoordInfo>) -> u8 {
    return [4 as u8,3,2,1,0].iter()
                            .map(|i| cal_bit(coord_queue, *i))
                            .reduce(|x, y| x | y)
                            .unwrap();
}
/// calculate 1 digit geo-bit from coordinate
/// returns u8
fn cal_bit(coord_queue: &mut VecDeque<CoordInfo>, i: u8) -> u8 {
    const LEFT: usize = 0;
    const RIGHT: usize = 1;
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