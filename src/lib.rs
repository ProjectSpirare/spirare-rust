pub mod debug;
pub mod transform;
pub mod time;
pub mod physics;

extern crate nalgebra as na;

use std::ops;

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

mod utils {
    pub fn str_to_ptr(text: &str) -> (usize, usize) {
        let ptr = text.as_ptr() as usize;
        let len = text.len();
        return (ptr, len);
    }
}

pub mod wasm_binding {
    #[repr(C)]
    pub struct Vector3 {
        pub x: f32,
        pub y: f32,
        pub z: f32,
    }

    #[repr(C)]
    pub struct Quaternion {
        pub x: f32,
        pub y: f32,
        pub z: f32,
        pub w: f32,
    }
}

mod common {
    pub use crate::wasm_binding;
    pub use crate::{Quaternion, Vector3};

    pub type ElementIndex = i32;
    pub type ResourceIndex = i32;

    pub fn vector3_to_wasm_vector3(vector: Vector3) -> wasm_binding::Vector3 {
        wasm_binding::Vector3 {
            x: vector.x,
            y: vector.y,
            z: vector.z,
        }
    }

    pub fn quaternion_to_wasm_quaternion(rotation: Quaternion) -> wasm_binding::Quaternion {
        wasm_binding::Quaternion {
            x: rotation.x,
            y: rotation.y,
            z: rotation.z,
            w: rotation.w,
        }
    }
}

pub mod element {
    use crate::common::ElementIndex;
    use crate::common::{ResourceIndex, Vector3};
    use crate::transform::Transform;
    use crate::{physics::Physics, transform};

    pub struct Element {
        index: ElementIndex,
    }

    impl Element {
        fn new(index: ElementIndex) -> Element {
            Element { index }
        }

        pub fn myself() -> Element {
            Element { index: 0 }
        }

        pub fn transform(&self) -> Transform {
            Transform::new(self.index)
        }

        pub fn physics(&self) -> Physics {
            Physics::new(self.index)
        }
    }

    pub fn spawn_object_by_id(resource_id: &str) -> Result<Element, &str> {
        let resource_index = get_resource_index_by_id(resource_id);
        if resource_index < 0 {
            Err("Resource not found")
        } else {
            let element_index = spawn_object(resource_index);
            if element_index < 0 {
                Err("Spawn failed")
            } else {
                Ok(Element::new(element_index))
            }
        }
    }

    pub fn spawn_object(resource_index: ResourceIndex) -> ElementIndex {
        unsafe {
            return element_spawn_object(resource_index);
        }
    }

    pub fn get_resource_index_by_id(resource_id: &str) -> ResourceIndex {
        unsafe {
            let ptr = resource_id.as_ptr() as usize;
            let len = resource_id.len();
            return element_get_resource_index_by_id(ptr, len);
        }
    }

    extern "C" {
        fn element_spawn_object(resource_id: i32) -> ElementIndex;
        fn element_get_resource_index_by_id(ptr: usize, len: usize) -> ResourceIndex;
    }
}
