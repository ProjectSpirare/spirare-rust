use crate::utils::quaternion_to_wasm_quaternion;
use crate::utils::vector3_to_wasm_vector3;
use crate::wasm_binding;
use crate::ElementIndex;
use crate::Quaternion;
use crate::Vector3;

pub struct Transform {
    index: ElementIndex,
}

impl Transform {
    pub fn get_local_position(&self) -> Vector3 {
        get_local_position(self.index)
    }
    pub fn set_local_position(&self, position: Vector3) {
        set_local_position(self.index, position)
    }
    pub fn get_world_position(&self) -> Vector3 {
        get_world_position(self.index)
    }
    pub fn set_world_position(&self, position: Vector3) {
        set_world_position(self.index, position)
    }
    pub fn get_local_rotation(&self) -> Quaternion {
        get_local_rotation(self.index)
    }
    pub fn set_local_rotation(&self, rotation: Quaternion) {
        set_local_rotation(self.index, rotation)
    }
    pub fn get_world_rotation(&self) -> Quaternion {
        get_world_rotation(self.index)
    }
    pub fn set_world_rotation(&self, rotation: Quaternion) {
        set_world_rotation(self.index, rotation)
    }

    pub fn new(index: ElementIndex) -> Transform {
        Transform { index }
    }

    pub fn myself() -> Transform {
        Transform::new(0)
    }

    pub fn forward(self) -> Vector3 {
        let q = self.get_world_rotation();
        q.forward()
    }
}

pub fn get_local_position(object_id: ElementIndex) -> Vector3 {
    unsafe {
        let x = transform_get_local_position_x(object_id);
        let y = transform_get_local_position_y(object_id);
        let z = transform_get_local_position_z(object_id);
        Vector3::new(x, y, z)
    }
}

pub fn set_local_position(object_id: ElementIndex, position: Vector3) {
    unsafe {
        let position = vector3_to_wasm_vector3(position);
        transform_set_local_position(object_id, position);
    }
}

pub fn get_world_position(object_id: ElementIndex) -> Vector3 {
    unsafe {
        let x = transform_get_world_position_x(object_id);
        let y = transform_get_world_position_y(object_id);
        let z = transform_get_world_position_z(object_id);
        Vector3::new(x, y, z)
    }
}

pub fn set_world_position(object_id: ElementIndex, position: Vector3) {
    unsafe {
        let position = vector3_to_wasm_vector3(position);
        transform_set_world_position(object_id, position);
    }
}

pub fn get_world_forward(object_id: ElementIndex) -> Vector3 {
    unsafe {
        let x = transform_get_world_forward_x(object_id);
        let y = transform_get_world_forward_y(object_id);
        let z = transform_get_world_forward_z(object_id);
        Vector3::new(x, y, z)
    }
}

pub fn get_local_rotation(object_id: ElementIndex) -> Quaternion {
    unsafe {
        let x = transform_get_local_rotation_x(object_id);
        let y = transform_get_local_rotation_y(object_id);
        let z = transform_get_local_rotation_z(object_id);
        let w = transform_get_local_rotation_w(object_id);
        Quaternion { x, y, z, w }
    }
}
pub fn set_local_rotation(object_id: ElementIndex, rotation: Quaternion) {
    unsafe {
        let rotation = quaternion_to_wasm_quaternion(rotation);
        transform_set_local_rotation(object_id, rotation);
    }
}

pub fn get_world_rotation(object_id: ElementIndex) -> Quaternion {
    unsafe {
        let x = transform_get_world_rotation_x(object_id);
        let y = transform_get_world_rotation_y(object_id);
        let z = transform_get_world_rotation_z(object_id);
        let w = transform_get_world_rotation_w(object_id);
        Quaternion { x, y, z, w }
    }
}

pub fn set_world_rotation(object_id: ElementIndex, rotation: Quaternion) {
    unsafe {
        let rotation = quaternion_to_wasm_quaternion(rotation);
        transform_set_world_rotation(object_id, rotation);
    }
}

pub fn get_local_scale(object_id: ElementIndex) -> Vector3 {
    unsafe {
        let x = transform_get_local_scale_x(object_id);
        let y = transform_get_local_scale_y(object_id);
        let z = transform_get_local_scale_z(object_id);
        Vector3::new(x, y, z)
    }
}
pub fn set_local_scale(object_id: ElementIndex, scale: Vector3) {
    unsafe {
        let scale = vector3_to_wasm_vector3(scale);
        transform_set_local_scale(object_id, scale);
    }
}
pub fn get_world_scale(object_id: ElementIndex) -> Vector3 {
    unsafe {
        let x = transform_get_world_scale_x(object_id);
        let y = transform_get_world_scale_y(object_id);
        let z = transform_get_world_scale_z(object_id);
        Vector3::new(x, y, z)
    }
}
pub fn set_world_scale(object_id: ElementIndex, scale: Vector3) {
    unsafe {
        let scale = vector3_to_wasm_vector3(scale);
        transform_set_world_scale(object_id, scale);
    }
}

extern "C" {
    fn transform_get_local_position_x(object_id: ElementIndex) -> f32;
    fn transform_get_local_position_y(object_id: ElementIndex) -> f32;
    fn transform_get_local_position_z(object_id: ElementIndex) -> f32;
    fn transform_set_local_position(object_id: ElementIndex, position: wasm_binding::Vector3);

    fn transform_get_world_position_x(object_id: ElementIndex) -> f32;
    fn transform_get_world_position_y(object_id: ElementIndex) -> f32;
    fn transform_get_world_position_z(object_id: ElementIndex) -> f32;
    fn transform_set_world_position(object_id: ElementIndex, position: wasm_binding::Vector3);

    fn transform_get_world_forward_x(object_id: ElementIndex) -> f32;
    fn transform_get_world_forward_y(object_id: ElementIndex) -> f32;
    fn transform_get_world_forward_z(object_id: ElementIndex) -> f32;

    fn transform_get_local_rotation_x(object_id: ElementIndex) -> f32;
    fn transform_get_local_rotation_y(object_id: ElementIndex) -> f32;
    fn transform_get_local_rotation_z(object_id: ElementIndex) -> f32;
    fn transform_get_local_rotation_w(object_id: ElementIndex) -> f32;
    fn transform_set_local_rotation(object_id: ElementIndex, rotation: wasm_binding::Quaternion);

    fn transform_get_world_rotation_x(object_id: ElementIndex) -> f32;
    fn transform_get_world_rotation_y(object_id: ElementIndex) -> f32;
    fn transform_get_world_rotation_z(object_id: ElementIndex) -> f32;
    fn transform_get_world_rotation_w(object_id: ElementIndex) -> f32;
    fn transform_set_world_rotation(object_id: ElementIndex, rotation: wasm_binding::Quaternion);

    fn transform_get_local_scale_x(object_id: ElementIndex) -> f32;
    fn transform_get_local_scale_y(object_id: ElementIndex) -> f32;
    fn transform_get_local_scale_z(object_id: ElementIndex) -> f32;
    fn transform_set_local_scale(object_id: ElementIndex, scale: wasm_binding::Vector3);

    fn transform_get_world_scale_x(object_id: ElementIndex) -> f32;
    fn transform_get_world_scale_y(object_id: ElementIndex) -> f32;
    fn transform_get_world_scale_z(object_id: ElementIndex) -> f32;
    fn transform_set_world_scale(object_id: ElementIndex, scale: wasm_binding::Vector3);
}
