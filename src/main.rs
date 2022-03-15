use window_handler::WindowHandler;

mod window_handler;
mod world;
mod object;

fn main() {
    WindowHandler::new(1600, 900, false).run();
}
