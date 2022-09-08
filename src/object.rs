// use glam::{ Vec2, Vec3, Vec4};
use obj::{load_obj, Obj, Position};
use std::{vec::Vec, io::{BufReader}, fs::File, path::Path};
use glium::{VertexBuffer, IndexBuffer};
use glium::index::PrimitiveType;
use glam::Vec3;
use crate::transform::{Transform};

use itertools::Itertools;

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub color: Option<[f32; 4]>
}

#[derive(Debug)]
pub struct Object {
    pub transform: Transform,
    pub vertices: Vec<Vertex>, 
    pub indices: Vec<u16>
}

impl Object {
    pub const fn new(transform: Transform, vertices: Vec<Vertex>, indices: Vec<u16>) -> Object {
        return Object{transform, vertices, indices};
    }

    pub fn from_file<P: AsRef<Path>>(path: P, transform: Transform) -> Result<Object,  Box<dyn std::error::Error>> {
        let input = BufReader::new(File::open(path)?);
        let obj: Obj<Position> = load_obj(input)?;

        let mut vertex_normal_buffer = Vec::new();
        vertex_normal_buffer.reserve(obj.vertices.len());

        for _ in 0..obj.vertices.len() {
            vertex_normal_buffer.push(Vec3::new(0.0, 0.0, 0.0));
        }

        for mut face in &obj.indices.iter().chunks(3) {
            let x = face.next().unwrap().clone() as usize;
            let y = face.next().unwrap().clone() as usize;
            let z = face.next().unwrap().clone() as usize;

            let vertex_1 = obj.vertices[x];
            let vertex_2 = obj.vertices[y];
            let vertex_3 = obj.vertices[z];

            let two_minus_one = Vec3::new(vertex_2.position[0] - vertex_1.position[0], vertex_2.position[1] - vertex_1.position[1], vertex_2.position[2] - vertex_1.position[2]);
            let three_minus_one = Vec3::new(vertex_3.position[0] - vertex_1.position[0], vertex_3.position[1] - vertex_1.position[1], vertex_3.position[2] - vertex_1.position[2]);

            let cross = two_minus_one.cross(three_minus_one);
            vertex_normal_buffer[x] += cross;
            vertex_normal_buffer[y] += cross; 
            vertex_normal_buffer[z] += cross; 
        }


        let mut vertices = Vec::new();
        vertices.reserve(obj.vertices.len());
        for (pos, normal) in itertools::zip(&obj.vertices, vertex_normal_buffer){
            // vertices.push(Vertex{position: pos.position, normal: [1.0, 0.0, 0.0], color: Option::None});
            vertices.push(Vertex{position: pos.position, normal: normal.normalize_or_zero().to_array(), color: Option::None});
        }

        //println!("{:?}", vertices);

        return Ok(Self::new(transform, vertices, obj.indices.clone()));
    }

    pub fn get_vertex_buffer(&self, display: &glium::Display) -> Result<glium::VertexBuffer<Vertex>, glium::vertex::BufferCreationError> {
        glium::implement_vertex!(Vertex, position, normal);
        return VertexBuffer::new(display, &self.vertices);
    }

    pub fn get_index_buffer(&self, display: &glium::Display) -> Result<glium::IndexBuffer<u16>, glium::index::BufferCreationError> {
        return IndexBuffer::new(display, PrimitiveType::TrianglesList, &self.indices);
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