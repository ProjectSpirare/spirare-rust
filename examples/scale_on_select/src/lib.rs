use clap::Clap;
use once_cell::sync::Lazy;

use spirare::transform::Transform;
use spirare::Vector3;

use spirare::element;

static DEFAULT_SIZE: f32 = 1.0;
static EXPANDED_SIZE: f32 = 2.0;

static mut EXPANDED: bool = false;
static mut EXPANDING: bool = false;
static mut SHRINKING: bool = false;

#[no_mangle]
unsafe fn update() {
    if !EXPANDING && !SHRINKING {
        return;
    }

    let transform = Transform::myself();
    let current_scale = transform.local_scale();

    let x = if EXPANDING {
        let x = current_scale.x + 1.0 * 0.01;
        if x > EXPANDED_SIZE {
            EXPANDING = false;
            EXPANDED = true;
            EXPANDED_SIZE
        } else {
            x
        }
    } else {
        let x = current_scale.x - 1.0 * 0.01;
        if x < DEFAULT_SIZE {
            SHRINKING = false;
            EXPANDED = false;
            DEFAULT_SIZE
        } else {
            x
        }
    };

    let scale = Vector3::new(x, x, x);
    transform.set_local_scale(scale);
}

#[no_mangle]
unsafe fn on_select() {
    if EXPANDING || (!SHRINKING && EXPANDED) {
        SHRINKING = true;
        EXPANDING = false;
        EXPANDED = false;
    } else {
        EXPANDING = true;
        SHRINKING = false;
        EXPANDED = false;
    }
}
