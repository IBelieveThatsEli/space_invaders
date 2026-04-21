mod app;
mod game;
mod gl;
mod input;
mod math;
mod renderer;
mod window;

use app::app::App;

fn main() {
    let mut app = App::new();
    app.run();
}
