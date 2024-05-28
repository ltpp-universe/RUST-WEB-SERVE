use crate::global::global::{ACCEPT_ENCODING, CONTENT_ENCODING, GZIP};
use crate::utils::tools;
use flate2::{write::GzEncoder, Compression};
use std::collections::HashMap;
use std::io::prelude::*;

/**
 * 使用gzip编码
 */
pub fn encoder(data: &Vec<u8>, level: u32) -> Vec<u8> {
    let mut encoder: GzEncoder<Vec<u8>> = GzEncoder::new(Vec::new(), Compression::new(level));
    match encoder.write_all(data) {
        Ok(_) => {}
        _ => {}
    }
    match encoder.finish() {
        Ok(compressed_data) => compressed_data,
        _ => data.to_vec(),
    }
}

/**
 * 判断是否需要开启gzip
 */
pub fn judge_need_open_gzip(
    client_header: &HashMap<String, String>,
    server_header: &HashMap<String, String>,
) -> bool {
    let client_encoding_value: String =
        tools::get_hash_map_one_value(&client_header, &ACCEPT_ENCODING.to_lowercase());
    let server_encoding_value: String =
        tools::get_hash_map_one_value(&server_header, &CONTENT_ENCODING.to_lowercase());
    let client_encoding_open: bool = client_encoding_value
        .to_lowercase()
        .contains(&GZIP.to_lowercase());
    let server_encoding_open: bool = server_encoding_value
        .to_lowercase()
        .contains(&GZIP.to_lowercase());
    if client_encoding_open && server_encoding_open {
        return true;
    }
    return false;
}
