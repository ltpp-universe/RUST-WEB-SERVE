use crate::global::global::DEFAULT_ERROR_HTML;

/**
 * 获取错误页的HTML
 */
pub fn get_error_html(text: &str) -> Vec<u8> {
    let html: Vec<u8> = DEFAULT_ERROR_HTML.replace("{}", text).as_bytes().to_vec();
    html
}
