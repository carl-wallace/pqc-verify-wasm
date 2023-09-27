#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)] // removed due to issue with Clap derive, clippy::unwrap_used)]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

mod verify;

use crate::verify::App;
use dioxus_web::Config;

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));

    log::debug!("PITTv3 start");

    dioxus_web::launch_cfg(App, Config::new());
}
