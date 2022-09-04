use glam::{f32, Mat4, Quat, Vec3};
use crate::samumath::degrees_to_radians;


pub struct Transform {
    scale: Vec3,
    rotation: Quat,
    translation: Vec3
}

impl Transform {
    pub fn to_mat4(&self) -> Mat4 {
        return Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.translation);
    }

    pub fn rotate_x(&mut self, degrees: f32) {
        self.rotation *= Quat::from_rotation_x(degrees_to_radians(degrees));
    }

    pub fn rotate_y(&mut self, degrees: f32) {
        self.rotation *= Quat::from_rotation_y(degrees_to_radians(degrees));
    }


    pub fn rotate_z(&mut self, degrees: f32) {
        self.rotation *= Quat::from_rotation_z(degrees_to_radians(degrees));
    }

    pub fn translate(&mut self, translation: Vec3) {
        self.translation += translation;
    }


    pub const fn new(scale: Vec3, rotation: Quat, translation: Vec3) -> Self {
        return Self{scale, rotation, translation};
    }

    pub const EMPTY: Self = Self::new(Vec3::ONE, Quat::IDENTITY, Vec3::ZERO);
}

