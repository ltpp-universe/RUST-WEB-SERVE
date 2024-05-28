use crate::config::config::Server;
use crate::utils::file;

pub fn get_ssl(server: &Server) -> (Vec<u8>, Vec<u8>) {
    let mut ssl_certificate: Vec<u8> = vec![];
    let mut ssl_certificate_key: Vec<u8> = vec![];
    let ssl_certificate_path: String = server.ssl_certificate_path.to_owned();
    let ssl_certificate_key_path: String = server.ssl_certificate_key_path.to_owned();
    match file::get_file_data(server, &ssl_certificate_path) {
        Some(tem_ssl_certificate) => {
            ssl_certificate = tem_ssl_certificate;
        }
        _ => {}
    }
    match file::get_file_data(server, &ssl_certificate_key_path) {
        Some(tem_ssl_certificate_key) => {
            ssl_certificate_key = tem_ssl_certificate_key;
        }
        _ => {}
    }
    (ssl_certificate, ssl_certificate_key)
}
