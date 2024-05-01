#![allow(warnings)]
mod base;
mod config;
mod global;
mod http;
mod print;
mod utils;
use base::base::Base;

fn main() {
    Base::run();
}
