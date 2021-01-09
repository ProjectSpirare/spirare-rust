use crate::wasm_binding;
use crate::Quaternion;
use crate::Vector3;

pub fn str_to_ptr(text: &str) -> (usize, usize) {
    let ptr = text.as_ptr() as usize;
    let len = text.len();
    return (ptr, len);
}

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
