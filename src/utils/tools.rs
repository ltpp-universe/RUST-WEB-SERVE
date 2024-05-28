use std::collections::HashMap;

/**
 * HashMap转String
 */
pub fn hash_map_to_string(hash_map: &HashMap<String, String>, br: &str) -> String {
    let mut result = String::new();
    for (key, value) in hash_map {
        result.push_str(&format!(
            "{}: {}{}",
            key.trim_start_matches(' '),
            value.trim_start_matches(' '),
            br
        ));
    }
    if !result.is_empty() {
        //  去除最后换行符
        result.truncate(result.len() - 1);
    }
    result
}

/**
 * 解析字符串数组转HashMap
 */
pub fn parse_string_array_to_hashmap(arr: &Vec<String>) -> HashMap<String, String> {
    let mut hashmap: HashMap<String, String> = HashMap::new();
    for item in arr {
        let parts: Vec<&str> = item.splitn(2, ':').collect();
        if parts.len() == 2 {
            let key: String = parts[0].trim_start_matches(' ').to_string();
            let value: String = parts[1].trim_start_matches(' ').to_string();
            hashmap.insert(key, value);
        }
    }
    hashmap
}

/**
 * Vuc<u8>转String
 */
pub fn vec_u8_to_string(vec: &Vec<u8>) -> String {
    match String::from_utf8(vec.clone()) {
        Ok(s) => s,
        Err(e) => "".to_owned(),
    }
}