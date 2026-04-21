use std::arch::x86_64::*;

// Common stuff

pub fn to_radians(deg: f32) -> f32 {
    let value = std::f32::consts::PI / 180.0;
    deg * value
}

pub fn to_radians_64(deg: f64) -> f64 {
    let value = std::f64::consts::PI / 180.0;
    deg * value
}

// Vector stuff

pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn from_simd(a: &__m128) -> Self {
        unsafe {
            let r: [f32; 4] = std::mem::transmute(*a);

            Self {
                x: r[0],
                y: r[1],
                z: r[2],
            }
        }
    }

    pub fn add(&self, other: &Self) -> Self {
        unsafe { Self::from_simd(&_mm_add_ps(self.to_simd(), other.to_simd())) }
    }

    pub fn sub(&self, other: &Self) -> Self {
        unsafe { Self::from_simd(&_mm_sub_ps(self.to_simd(), other.to_simd())) }
    }

    pub fn dot(a: &Self, b: &Self) -> f32 {
        unsafe {
            let va = Self::to_simd(a);
            let vb = Self::to_simd(b);

            let mul = _mm_mul_ps(va, vb);
            let shuf = _mm_shuffle_ps(mul, mul, 0b11_10_01_00);
            let sums = _mm_add_ps(mul, shuf);
            let shuf2 = _mm_shuffle_ps(sums, sums, 0b01_00_11_10);
            let final_sum = _mm_add_ps(sums, shuf2);

            _mm_cvtss_f32(final_sum)
        }
    }

    pub fn cross(a: &Self, b: &Self) -> Self {
        unsafe {
            let va = Self::to_simd(a);
            let vb = Self::to_simd(b);

            let a_yzx = _mm_shuffle_ps(va, va, 0b11_00_10_01);
            let b_zxy = _mm_shuffle_ps(vb, vb, 0b11_01_00_10);
            let a_zxy = _mm_shuffle_ps(va, va, 0b11_01_00_10);
            let b_yzx = _mm_shuffle_ps(vb, vb, 0b11_00_10_01);

            let mul1 = _mm_mul_ps(a_yzx, b_zxy);
            let mul2 = _mm_mul_ps(a_zxy, b_yzx);

            let result = _mm_sub_ps(mul1, mul2);
            let r: [f32; 4] = std::mem::transmute(result);

            Self {
                x: r[0],
                y: r[1],
                z: r[2],
            }
        }
    }

    pub fn normalize(a: &Self) -> Self {
        unsafe {
            let len = Self::dot(a, a).sqrt();
            let inv = _mm_set1_ps(1.0 / len);

            let va = Self::to_simd(a);
            let result = _mm_mul_ps(va, inv);
            let r: [f32; 4] = std::mem::transmute(result);

            Self {
                x: r[0],
                y: r[1],
                z: r[2],
            }
        }
    }

    pub fn to_simd(&self) -> __m128 {
        unsafe { _mm_set_ps(0.0, self.z, self.y, self.x) }
    }
}

pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub j: f32,
}

impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32, j: f32) -> Self {
        Self { x, y, z, j }
    }

    pub fn from_simd(a: &__m128) -> Self {
        unsafe {
            let r: [f32; 4] = std::mem::transmute(*a);

            Self {
                x: r[0],
                y: r[1],
                z: r[2],
                j: r[3],
            }
        }
    }

    pub fn to_simd(&self) -> __m128 {
        unsafe { _mm_set_ps(self.j, self.z, self.y, self.x) }
    }
}

// Matrix stuff
#[derive(Copy, Clone)]
pub struct Mat4 {
    cols: [__m128; 4],
}

impl Mat4 {
    pub fn identity() -> Self {
        unsafe {
            Self {
                cols: [
                    _mm_setr_ps(1.0, 0.0, 0.0, 0.0),
                    _mm_setr_ps(0.0, 1.0, 0.0, 0.0),
                    _mm_setr_ps(0.0, 0.0, 1.0, 0.0),
                    _mm_setr_ps(0.0, 0.0, 0.0, 1.0),
                ],
            }
        }
    }

    pub fn perspective(fovy: f32, aspect: f32, z_near: f32, z_far: f32) -> Self {
        unsafe {
            let f = 1.0 / (0.5 * fovy).tan();

            let nf = 1.0 / (z_near - z_far);

            Self {
                cols: [
                    _mm_setr_ps(f / aspect, 0.0, 0.0, 0.0),
                    _mm_setr_ps(0.0, f, 0.0, 0.0),
                    _mm_setr_ps(0.0, 0.0, (z_far + z_near) * nf, -1.0),
                    _mm_setr_ps(0.0, 0.0, (2.0 * z_far * z_near) * nf, 0.0),
                ],
            }
        }
    }

    pub fn look_at(eye: &Vec3, target: &Vec3, up: &Vec3) -> Self {
        unsafe {
            let f = Vec3::normalize(&Vec3::sub(target, eye));
            let r = Vec3::normalize(&Vec3::cross(&f, up));
            let u = Vec3::cross(&r, &f);

            let neg_f = _mm_sub_ps(_mm_setzero_ps(), Vec3::to_simd(&f));

            let tx = -Vec3::dot(&r, &eye);
            let ty = -Vec3::dot(&u, &eye);
            let tz = Vec3::dot(&f, &eye);

            Self {
                cols: [
                    Vec3::to_simd(&r),
                    Vec3::to_simd(&u),
                    neg_f,
                    _mm_set_ps(1.0, tz, ty, tx),
                ],
            }
        }
    }

    pub fn translate(mut self, v: &Vec3) -> Self {
        unsafe {
            let vx = _mm_shuffle_ps(Vec3::to_simd(v), Vec3::to_simd(v), 0x00);
            let vy = _mm_shuffle_ps(Vec3::to_simd(v), Vec3::to_simd(v), 0x55);
            let vz = _mm_shuffle_ps(Vec3::to_simd(v), Vec3::to_simd(v), 0xAA);

            let mut result = self.cols[3];

            result = _mm_add_ps(result, _mm_mul_ps(self.cols[0], vx));
            result = _mm_add_ps(result, _mm_mul_ps(self.cols[1], vy));
            result = _mm_add_ps(result, _mm_mul_ps(self.cols[2], vz));

            self.cols[3] = result;

            self
        }
    }

    pub fn scale(mut self, v: &Vec3) -> Self {
        unsafe {
            let sx = _mm_shuffle_ps(Vec3::to_simd(v), Vec3::to_simd(v), 0x00);
            let sy = _mm_shuffle_ps(Vec3::to_simd(v), Vec3::to_simd(v), 0x55);
            let sz = _mm_shuffle_ps(Vec3::to_simd(v), Vec3::to_simd(v), 0xAA);

            self.cols[0] = _mm_mul_ps(self.cols[0], sx);
            self.cols[1] = _mm_mul_ps(self.cols[1], sy);
            self.cols[2] = _mm_mul_ps(self.cols[2], sz);

            self
        }
    }

    pub fn rotate(mut self, v: &Vec3) -> Self {
        unsafe {
            let mut angles = [0.0f32; 4];
            _mm_storeu_ps(angles.as_mut_ptr(), v.to_simd());

            let (x, y, z) = (angles[0], angles[1], angles[2]);

            let (sx, cx) = x.sin_cos();
            let (sy, cy) = y.sin_cos();
            let (sz, cz) = z.sin_cos();

            let r0 = Vec4::new(cy * cz, cy * sz, -sy, 0.0);
            let r1 = Vec4::new(sx * sy * cz - cx * sz, sx * sy * sz + cx * cz, sx * cy, 0.0);
            let r2 = Vec4::new(cx * sy * cz + sx * sz, cx * sy * sz - sx * cz, cx * cy, 0.0);
            // let r0 = _mm_setr_ps(cy * cz, cy * sz, -sy, 0.0);
            // let r1 = _mm_setr_ps(sx * sy * cz - cx * sz, sx * sy * sz + cx * cz, sx * cy, 0.0);
            // let r2 = _mm_setr_ps(cx * sy * cz + sx * sz, cx * sy * sz - sx * cz, cx * cy, 0.0);

            self.cols[0] = Self::mul_vec4(&self, &r0).to_simd();
            self.cols[1] = Self::mul_vec4(&self, &r1).to_simd();
            self.cols[2] = Self::mul_vec4(&self, &r2).to_simd();

            self
        }
    }

    pub fn value_ptr(&self) -> *const f32 {
        self.cols.as_ptr() as *const f32
    }

    pub fn mul(&self, other: &Self) -> Self {
        Self {
            cols: [
                Self::mul_col(&self, &Vec4::from_simd(&other.cols[0])).to_simd(),
                Self::mul_col(&self, &Vec4::from_simd(&other.cols[1])).to_simd(),
                Self::mul_col(&self, &Vec4::from_simd(&other.cols[2])).to_simd(),
                Self::mul_col(&self, &Vec4::from_simd(&other.cols[3])).to_simd(),
            ],
        }
    }

    fn mul_vec4(&self, v: &Vec4) -> Vec4 {
        unsafe {
            let xxxx = _mm_shuffle_ps(Vec4::to_simd(v), Vec4::to_simd(v), 0x00);
            let yyyy = _mm_shuffle_ps(Vec4::to_simd(v), Vec4::to_simd(v), 0x55);
            let zzzz = _mm_shuffle_ps(Vec4::to_simd(v), Vec4::to_simd(v), 0xAA);

            let mut result = _mm_mul_ps(self.cols[0], xxxx);
            result = _mm_add_ps(result, _mm_mul_ps(self.cols[1], yyyy));
            result = _mm_add_ps(result, _mm_mul_ps(self.cols[2], zzzz));

            Vec4::from_simd(&result)
        }
    }

    fn mul_col(&self, col: &Vec4) -> Vec4 {
        unsafe {
            let xxxx = _mm_shuffle_ps(col.to_simd(), col.to_simd(), 0x00);
            let yyyy = _mm_shuffle_ps(col.to_simd(), col.to_simd(), 0x55);
            let zzzz = _mm_shuffle_ps(col.to_simd(), col.to_simd(), 0xAA);
            let wwww = _mm_shuffle_ps(col.to_simd(), col.to_simd(), 0xFF);

            // Multiply and accumulate
            let mul0 = _mm_mul_ps(self.cols[0], xxxx);
            let mul1 = _mm_mul_ps(self.cols[1], yyyy);
            let mul2 = _mm_mul_ps(self.cols[2], zzzz);
            let mul3 = _mm_mul_ps(self.cols[3], wwww);

            let sum0 = _mm_add_ps(mul0, mul1);
            let sum1 = _mm_add_ps(mul2, mul3);

            Vec4::from_simd(&_mm_add_ps(sum0, sum1))
        }
    }
}
