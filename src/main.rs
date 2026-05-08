mod app;
mod audio;
mod game;
mod gl;
mod gltf;
mod input;
mod json;
mod math;
mod renderer;
mod window;

use app::app::App;

fn main() {
    let mut app = App::new();
    app.run();
}
