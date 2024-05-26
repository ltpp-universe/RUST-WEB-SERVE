#![allow(warnings)]
mod base;
mod config;
mod global;
mod http;
mod log;
mod print;
mod proxy;
mod ssl;
mod template;
mod utils;
use base::base::Base;

fn main() {
    Base::run();
}
