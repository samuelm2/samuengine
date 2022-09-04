use glam::{IVec3, Vec2, Vec3, Vec4};
use obj::{load_obj, Obj, Position};
use std::{vec::Vec, io::{BufReader}, fs::File, path::Path};
use crate::transform::{Transform};


pub struct Vertex {
    pub position: Vec3,
    pub normal: Option<Vec2>,
    pub color: Option<Vec4>
}

pub struct Object {
    pub transform: Transform,
    pub vertices: Vec<Vertex>, 
    pub indices: Vec<i16>

}

impl Object {
    pub const fn new(transform: Transform, vertices: Vec<Vertex>, indices: Vec<IVec3>) -> Object {
        return Object{transform, vertices, indices};
    }

    pub fn from_file(path: &Path, transform: Transform) -> Result<Object,  dyn std::error::Error> {
        let input = BufReader::new(File::open(path)?);
        let obj: Obj<Position> = load_obj(input)?;

        let vertices = Vec::new();
        vertices.reserve(obj.vertices.len());
        for pos in obj.vertices {
            vertices.push(Vertex{position: Vec3::from_array(pos.position), normal: Option::None, color: Option::None});
        }

        // let indices = Vec::new();
        // indices.reserve(obj.indices.len());

        return Self::New(transform, vertices, obj.indices);
    }

}

// impl Camera {
//     pub fn new(transform: Transform, near: f32, far: f32, fov_degrees: f32, aspect_ratio: f32) -> Camera {
//         return Camera{transform, near, far, fov: degrees_to_radians(fov_degrees), aspect_ratio};
//     }

//     pub fn to_view_matrix(&self) -> Mat4 {
//         let transform_matrix = self.transform.to_mat4();
//         return transform_matrix.inverse()
//     }

//     pub fn to_perspective_matrix(&self) -> Mat4 {
//         return glam::Mat4::perspective_rh(self.fov, self.aspect_ratio, self.near, self.far);
//     }
// }