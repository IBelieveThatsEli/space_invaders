use super::engine::Engine;
use super::time::Time;

pub struct App {
    engine: Engine,
    time: Time,
}

impl App {
    pub fn new() -> Self {
        let engine = Engine::new();
        let time = Time::new();

        Self { engine, time }
    }
    pub fn run(&mut self) {
        while self.engine.is_open() {
            self.time.update();
            self.engine.update(self.time.delta());
        }
    }
}
