pub mod debug;
pub mod element;
pub mod physics;
pub mod time;
pub mod transform;
mod utils;
mod wasm_binding;

extern crate nalgebra as na;

use std::ops;

pub type ElementIndex = i32;
pub type ResourceIndex = i32;

// Vector3
#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x, y, z }
    }

    pub fn forward() -> Vector3 {
        Vector3::new(1.0, 0.0, 0.0)
    }

    fn na_vec(self) -> na::Vector3<f32> {
        na::Vector3::new(self.x, self.y, self.z)
    }

    fn from_na_vec(v: na::Vector3<f32>) -> Vector3 {
        Vector3::new(v.x, v.y, v.z)
    }
}

impl ops::Add for Vector3 {
    type Output = Vector3;
    fn add(self, v: Vector3) -> Vector3 {
        Vector3::new(self.x + v.x, self.y + v.y, self.z + v.z)
    }
}

impl ops::Mul<f32> for Vector3 {
    type Output = Vector3;
    fn mul(self, scale: f32) -> Self::Output {
        Vector3::new(self.x * scale, self.y * scale, self.z * scale)
    }
}

impl ops::Mul<Vector3> for f32 {
    type Output = Vector3;
    fn mul(self, vector: Vector3) -> Self::Output {
        Vector3::new(self * vector.x, self * vector.y, self * vector.z)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Quaternion {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Quaternion {
        Quaternion { x, y, z, w }
    }

    pub fn forward(self) -> Vector3 {
        let q = self.na_quat();
        let v = q.transform_vector(&Vector3::forward().na_vec());
        Vector3::from_na_vec(v)
    }

    fn na_quat(self) -> na::UnitQuaternion<f32> {
        let q = na::Quaternion::new(self.w, self.x, self.y, self.z);
        na::UnitQuaternion::from_quaternion(q)
    }
}
