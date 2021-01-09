use crate::utils::vector3_to_wasm_vector3;
use crate::wasm_binding;
use crate::ElementIndex;
use crate::Vector3;
/*
use crate::{
    // common::{vector3_to_wasm_vector3, ElementIndex},
    wasm_binding, Vector3,
};
*/

pub struct Physics {
    index: ElementIndex,
}

impl Physics {
    pub(crate) fn new(index: ElementIndex) -> Physics {
        Physics { index }
    }

    pub fn myself() -> Physics {
        Physics::new(0)
    }

    pub fn set_world_velocity(self, velocity: Vector3) {
        set_world_velocity(self.index, velocity)
    }
}

fn set_world_velocity(object_id: ElementIndex, velocity: Vector3) {
    unsafe {
        let velocity = vector3_to_wasm_vector3(velocity);
        return physics_set_world_velocity(object_id, velocity);
    }
}

extern "C" {
    fn physics_set_world_velocity(object_id: ElementIndex, velocity: wasm_binding::Vector3);
}
