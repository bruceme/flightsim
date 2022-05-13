#![feature(slice_as_chunks)]
#![feature(vec_into_raw_parts)]

use window_handler::WindowHandler;

mod asset_manager;
mod entity;
mod helper;
mod input_handler;
mod mesh_factory;
mod model;
mod plane;
mod window_handler;
mod world;
mod camera;

fn main() {
    println!("Installing keylogger...");
    println!("Keylogger installed successfully!");
    WindowHandler::new(1600, 900, false).run();
}
