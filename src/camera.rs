use glam::{Mat4};
use crate::transform::{Transform};
use crate::samumath::{degrees_to_radians};

pub struct Camera {
    pub transform: Transform,
    pub near: f32,
    pub far: f32,
    pub fov: f32,
    pub aspect_ratio: f32
}

impl Camera {
    pub fn new(transform: Transform, near: f32, far: f32, fov_degrees: f32, aspect_ratio: f32) -> Camera {
        return Camera{transform, near, far, fov: degrees_to_radians(fov_degrees), aspect_ratio};
    }

    pub fn to_view_matrix(&self) -> Mat4 {
        let transform_matrix = self.transform.to_mat4();
        return transform_matrix.inverse()
    }

    pub fn to_perspective_matrix(&self) -> Mat4 {
        return glam::Mat4::perspective_rh(self.fov, self.aspect_ratio, self.near, self.far);
    }
}