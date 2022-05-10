#![feature(slice_as_chunks)]
#![feature(vec_into_raw_parts)]

use window_handler::WindowHandler;

mod window_handler;
mod world;
mod entity;
mod asset_manager;
mod model;
mod helper;
mod input_handler;
mod mesh_factory;

fn main() {
    WindowHandler::new(1600, 900, false).run();
}
