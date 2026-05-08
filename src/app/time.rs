use std::time::Instant;

fn now(start: &Instant) -> f64 {
    start.elapsed().as_secs_f64()
}

pub struct Time {
    start: Instant,
    last: f64,
    delta: f64,
}

impl Time {
    pub fn new() -> Self {
        let start = Instant::now();

        Self {
            start,
            last: 0.0,
            delta: 0.0,
        }
    }

    #[inline]
    pub fn update(&mut self) {
        let current = now(&self.start);

        self.delta = current - self.last;
        self.last = current;
    }

    #[inline]
    pub fn delta(&self) -> f64 {
        self.delta
    }
}
