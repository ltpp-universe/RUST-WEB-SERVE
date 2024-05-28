use crate::global::global::{ACCEPT_ENCODING, GZIP};
use flate2::{write::GzEncoder, Compression};
use std::collections::HashMap;
use std::io::prelude::*;

/**
 * 使用gzip编码
 */
pub fn encoder(data: &Vec<u8>) -> Vec<u8> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data).unwrap();
    let compressed_data: Vec<u8> = encoder.finish().unwrap();
    return compressed_data;
}

/**
 * 判断是否需要开启gzip
 */
pub fn judge_need_open_gzip(header: &HashMap<String, String>) -> bool {
    match header.get(&ACCEPT_ENCODING.to_lowercase()) {
        Some(value) => {
            let lower_value = value.to_lowercase();
            lower_value.contains(&GZIP.to_lowercase())
        }
        _ => false,
    }
}
