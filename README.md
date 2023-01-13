# geohash-rs

A geohash crate for rust

## Examples

### Encoder

```rust
use geohash_rs;
let geohash = geohash_rs::encode(37.5666805, 126.9784147, 8);

assert_eq!(geohash, String::from("wydm9qyc"))
```

### Decoder

```rust
use geohash_rs;
let gps_bounds = geohash_rs::decode(String::from("wydm9qyc"));

assert_eq!(gps_bounds.latitude[0], 37.56654739379883);
assert_eq!(gps_bounds.latitude[1], 37.56671905517578);
assert_eq!(gps_bounds.longitude[0], 126.97826385498047);
assert_eq!(gps_bounds.longitude[1], 126.97860717773438);
```
