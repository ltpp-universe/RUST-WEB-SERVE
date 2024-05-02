use crate::global::global::DEFAULT_ERROR_HTML;

/**
 * 获取错误页的HTML
 */
pub fn get_error_html(text: &str) -> String {
    let html: String = DEFAULT_ERROR_HTML.replace("{}", text);
    html
}
