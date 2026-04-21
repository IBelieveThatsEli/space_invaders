#[repr(C)]
struct timespec {
    tv_sec: i64,
    tv_nsec: i64,
}

unsafe extern "C" {
    fn clock_gettime(clk_id: i32, tp: *mut timespec) -> i32;
}

const CLOCK_MONOTONIC: i32 = 1;

fn now() -> f64 {
    unsafe {
        let mut ts = timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        clock_gettime(CLOCK_MONOTONIC, &mut ts);

        ts.tv_sec as f64 + (ts.tv_nsec as f64 * 1e-9)
    }
}

pub struct Time {
    last: f64,
    delta: f64,
}

impl Time {
    pub fn new() -> Self {
        let t = now();
        Self {
            last: t,
            delta: 0.0,
        }
    }

    #[inline]
    pub fn update(&mut self) {
        let current = now();
        self.delta = current - self.last;
        self.last = current;
    }

    #[inline]
    pub fn delta(&self) -> f64 {
        self.delta
    }
}
