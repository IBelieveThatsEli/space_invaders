use std::arch::x86_64::*;

// Common stuff

pub fn to_radians(deg: f32) -> f32 {
    let value = std::f32::consts::PI / 180.0;
    deg * value
}

// pub fn to_radians_64(deg: f64) -> f64 {
//     let value = std::f64::consts::PI / 180.0;
//     deg * value
// }
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

    pub fn lerp(a: Self, b: Self, t: f32) -> Self {
        Self {
            x: a.x + (b.x - a.x) * t,
            y: a.y + (b.y - a.y) * t,
            z: a.z + (b.z - a.z) * t,
        }
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

    pub fn from_euler(euler: &Vec3) -> Self {
        let (sx, cx) = (euler.x * 0.5).sin_cos();
        let (sy, cy) = (euler.y * 0.5).sin_cos();
        let (sz, cz) = (euler.z * 0.5).sin_cos();

        Self {
            x: sx * cy * cz - cx * sy * sz,
            y: cx * sy * cz + sx * cy * sz,
            z: cx * cy * sz - sx * sy * cz,
            j: cx * cy * cz + sx * sy * sz,
        }
    }

    pub fn lerp(a: Self, b: Self, t: f32) -> Self {
        Self {
            x: a.x + (b.x - a.x) * t,
            y: a.y + (b.y - a.y) * t,
            z: a.z + (b.z - a.z) * t,
            j: a.j + (b.j - a.j) * t,
        }
    }

    pub fn normalize_quat(q: Self) -> Self {
        let len = (q.x * q.x + q.y * q.y + q.z * q.z + q.j * q.j).sqrt();
        if len == 0.0 {
            Self {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                j: 1.0,
            }
        } else {
            Self {
                x: q.x / len,
                y: q.y / len,
                z: q.z / len,
                j: q.j / len,
            }
        }
    }
}

// Matrix stuff
#[derive(Copy, Clone)]
pub struct Mat4 {
    pub cols: [__m128; 4],
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
    pub fn rotate_quat(self, q: &Vec4) -> Self {
        unsafe {
            let xx = q.x * q.x;
            let yy = q.y * q.y;
            let zz = q.z * q.z;
            let xy = q.x * q.y;
            let xz = q.x * q.z;
            let yz = q.y * q.z;
            let wx = q.j * q.x;
            let wy = q.j * q.y;
            let wz = q.j * q.z;

            let rot = Mat4 {
                cols: [
                    _mm_setr_ps(1.0 - 2.0 * (yy + zz), 2.0 * (xy + wz), 2.0 * (xz - wy), 0.0),
                    _mm_setr_ps(2.0 * (xy - wz), 1.0 - 2.0 * (xx + zz), 2.0 * (yz + wx), 0.0),
                    _mm_setr_ps(2.0 * (xz + wy), 2.0 * (yz - wx), 1.0 - 2.0 * (xx + yy), 0.0),
                    _mm_setr_ps(0.0, 0.0, 0.0, 1.0),
                ],
            };

            self.mul(&rot)
        }
    }

    pub fn rotate(self, v: &Vec3) -> Self {
        let q = Vec4::from_euler(v);
        self.rotate_quat(&q)
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

    // fn mul_vec4(&self, v: &Vec4) -> Vec4 {
    //     unsafe {
    //         let xxxx = _mm_shuffle_ps(Vec4::to_simd(v), Vec4::to_simd(v), 0x00);
    //         let yyyy = _mm_shuffle_ps(Vec4::to_simd(v), Vec4::to_simd(v), 0x55);
    //         let zzzz = _mm_shuffle_ps(Vec4::to_simd(v), Vec4::to_simd(v), 0xAA);

    //         let mut result = _mm_mul_ps(self.cols[0], xxxx);
    //         result = _mm_add_ps(result, _mm_mul_ps(self.cols[1], yyyy));
    //         result = _mm_add_ps(result, _mm_mul_ps(self.cols[2], zzzz));

    //         Vec4::from_simd(&result)
    //     }
    // }

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
