#![feature(slice_as_chunks)]
#![feature(vec_into_raw_parts)]

use window_handler::WindowHandler;

mod asset_manager;
mod entity;
mod helper;
mod input_handler;
<<<<<<< HEAD
mod mesh_factory;
=======
mod model;
mod window_handler;
mod world;
>>>>>>> 928e71403e5375d67113a0104c37baff017bbb85

fn main() {
    WindowHandler::new(1600, 900, false).run();
}
