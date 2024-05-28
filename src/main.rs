#![allow(warnings)]
mod config;
mod file_safe;
mod global;
mod gzip;
mod http;
mod log;
mod print;
mod proxy;
mod request;
mod ssl;
mod template;
mod utils;
use request::request::Request;

fn main() {
    Request::run();
}
