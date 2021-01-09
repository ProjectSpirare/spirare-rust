use crate::physics::Physics;
use crate::transform::Transform;
use crate::ElementIndex;
use crate::ResourceIndex;

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

fn spawn_object(resource_index: ResourceIndex) -> ElementIndex {
    unsafe {
        return element_spawn_object(resource_index);
    }
}

fn get_resource_index_by_id(resource_id: &str) -> ResourceIndex {
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
