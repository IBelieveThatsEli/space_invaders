use super::engine::Engine;

pub struct App {
    engine: Engine,
}

impl App {
    pub fn new() -> Self {
        let engine = Engine::new();

        Self { engine }
    }
    pub fn run(&mut self) {
        while self.engine.is_open() {
            self.engine.update(0.0);
        }
    }
}
