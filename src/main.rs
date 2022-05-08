use window_handler::WindowHandler;

mod asset_manager;
mod entity;
mod helper;
mod input_handler;
mod model;
mod window_handler;
mod world;

fn main() {
    WindowHandler::new(1600, 900, false).run();
}
