use rmath::{Mat4, Vec3};

pub struct Camera {
    pub position: Vec3,
    pub fovy: f32,
    pub aspect: f32,
    pub z_near: f32,
    pub z_far: f32,
}

impl Camera {
    pub fn new(position: Vec3, fovy: f32, aspect: f32, z_near: f32, z_far: f32) -> Self {
        Self {
            position,
            fovy,
            aspect,
            z_near,
            z_far,
        }
    }
    pub fn get_pv(&self) -> Mat4 {
        let mut view = Mat4::identity();
        view.translate(&self.position);

        let projection = Mat4::perspective(self.fovy, self.aspect, self.z_near, self.z_far);

        projection.mul(&view)
    }
}
