mod config;
mod file_safe;
mod global;
mod gzip;
mod http;
mod log;
mod print;
mod proxy;
mod request;
mod template;
mod utils;
use request::request::Request;

fn main() {
    Request::run();
}
