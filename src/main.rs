use window_handler::WindowHandler;

mod window_handler;
mod world;
mod entity;
mod asset_manager;
mod model;
mod helper;

fn main() {
    WindowHandler::new(1600, 900, false).run();
}
