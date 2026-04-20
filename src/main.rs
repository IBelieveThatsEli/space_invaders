mod app;
mod gl;
mod input;
mod renderer;
mod window;

use app::app::App;

fn main() {
    let mut app = App::new();
    app.run();
}
