use rmath::{mat4::Mat4, vec3::*};

pub struct Transform {
    pub position: Vec3,
    pub rotation: Vec3, // Euler angles in radians
    pub scale: Vec3,
}

impl Transform {
    pub fn new(position: Vec3, rotation: Vec3, scale: Vec3) -> Self {
        Self {
            position,
            rotation,
            scale,
        }
    }

    pub fn identity() -> Self {
        Self {
            position: Vec3::init(0.0, 0.0, 0.0),
            rotation: Vec3::init(0.0, 0.0, 0.0),
            scale: Vec3::init(1.0, 1.0, 1.0),
        }
    }

    /// Generate model matrix from transform
    pub fn get_model_matrix(&self) -> Mat4 {
        let mut model = Mat4::identity();
        model.translate(&self.position);

        // Rotate (order: Y -> X -> Z)
        let rot = extract(self.rotation.simd);
        if rot[1] != 0.0 {
            model.rotate(&Vec3::init(0.0, rot[1], 0.0));
        }
        if rot[0] != 0.0 {
            model.rotate(&Vec3::init(rot[0], 0.0, 0.0));
        }
        if rot[2] != 0.0 {
            model.rotate(&Vec3::init(0.0, 0.0, rot[2]));
        }

        // Scale
        model.scale(&self.scale);

        model
    }

    pub fn translate(&mut self, delta: Vec3) {
        self.position.add(&delta);
    }

    pub fn rotate(&mut self, delta: Vec3) {
        self.rotation.add(&delta);
    }

    pub fn set_scale(&mut self, scale: Vec3) {
        self.scale = scale;
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::identity()
    }
}
