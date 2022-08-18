//! 'geohash' is a geohash encoder for rust.

use std::collections::VecDeque;
/// Encodes gps coordinates to String
/// 
/// # Examples
/// 
/// ```
/// let geohash = geohash::encode(37.5666805, 126.9784147, 8);
/// 
/// assert_eq!(geohash, String::from("wydm9qyc"))
/// ```
pub fn encode(latitude: f32, longitude: f32, length_geohash: u32) -> String {
    let lat_info = CoordInfo {
        bound: [-90.0,90.0],
        coord: latitude
    };
    let long_info = CoordInfo {
        bound: [-180.0,180.0],
        coord: longitude
    };
    let mut coord_queue = VecDeque::from([long_info, lat_info]);
    let geohash_string = (0..length_geohash).into_iter()
                                                    .map(|_| get_geo_bit(&mut coord_queue))
                                                    .map(|a| encode_b32(&a))
                                                    .collect();
    return geohash_string;
}
fn get_geo_bit(coord_queue: &mut VecDeque<CoordInfo>) -> u8 {
    return [4,3,2,1,0].iter()
    .map(|i| cal_bit(coord_queue, *i))
    .reduce(|x, y| x | y)
    .unwrap();
}
fn cal_bit(coord_queue: &mut VecDeque<CoordInfo>, i: u32) -> u8 {
    const LEFT: usize = 0;
    const RIGHT: usize = 1;
    let mut coord_info_inst = coord_queue.pop_front().unwrap();
    let mid = &(coord_info_inst.bound).iter().sum::<f32>() / 2.0;
    if &mid <= &(coord_info_inst.coord) {
        coord_info_inst.bound = [mid, coord_info_inst.bound[RIGHT]];
        coord_queue.push_back(coord_info_inst);
        return u8::pow(2, i);
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
fn encode_b32(i: &u8) -> char {
    if i == &(0u8) {
        return '0';
    } else if i == &(1) {
        return '1';
    } else if i == &(2) {
        return '2';
    } else if i == &(3) {
        return '3';
    } else if i == &(4) {
        return '4';
    } else if i == &(5) {
        return '5';
    } else if i == &(6) {
        return '6';
    } else if i == &(7) {
        return '7';
    } else if i == &(8) {
        return '8';
    } else if i == &(9) {
        return '9';
    } else if i == &(10) {
        return 'b';
    } else if i == &(11) {
        return 'c';
    } else if i == &(12) {
        return 'd';
    } else if i == &(13) {
        return 'e';
    } else if i == &(14) {
        return 'f';
    } else if i == &(15) {
        return 'g';
    } else if i == &(16) {
        return 'h';
    } else if i == &(17) {
        return 'j';
    } else if i == &(18) {
        return 'k';
    } else if i == &(19) {
        return 'm';
    } else if i == &(20) {
        return 'n';
    } else if i == &(21) {
        return 'p';
    } else if i == &(22) {
        return 'q';
    } else if i == &(23) {
        return 'r';
    } else if i == &(24) {
        return 's';
    } else if i == &(25) {
        return 't';
    } else if i == &(26) {
        return 'u';
    } else if i == &(27) {
        return 'v';
    } else if i == &(28) {
        return 'w';
    } else if i == &(29) {
        return 'x';
    } else if i == &(30) {
        return 'y';
    } else if i == &(31) {
        return 'z';
    } else {
        return 'a';
    }
}