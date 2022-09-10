use glam::{Vec4};
use crate::transform::{Transform};

pub struct Light {
    pub transform: Transform,
    pub ambient_color: Vec4,
    pub diffuse_color: Vec4,
    pub specular_color: Vec4,
}

impl Light {
    pub const fn new(transform: Transform, ambient_color: Vec4, diffuse_color: Vec4, specular_color: Vec4) -> Light {
        return Light{transform, ambient_color, diffuse_color, specular_color};
    }


    pub const WHITE: Light = Light{transform: Transform::EMPTY, ambient_color: Vec4::ZERO, diffuse_color: Vec4::ONE, specular_color: Vec4::ONE};
}