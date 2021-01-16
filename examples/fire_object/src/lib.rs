use clap::Clap;
use once_cell::sync::Lazy;

use spirare::transform::Transform;
use spirare::Vector3;

use spirare::element;
/*
use element::Element;
use transform::Transform;
use uni_wasm::element;
use uni_wasm::physics;
use uni_wasm::{transform, Vector3};
*/

/*
fn main() {
    let bullet_index = element::get_resource_index_by_id("123");
    print!("{}", bullet_index);
}
*/

#[no_mangle]
unsafe fn on_use() {
    let transform = Transform::myself();
    let gun_position = transform.world_position();
    let gun_rotation = transform.world_rotation();

    // spawn bullet
    let result = element::spawn_object_by_id("bullet");
    if result.is_err() {
        return;
    }
    let bullet_element = result.unwrap();

    // set position and rotation
    let offset = 0.5;
    let forward = transform.forward();
    let bullet_position = gun_position + offset * forward;

    let bullet_transform = bullet_element.transform();
    bullet_transform.set_world_position(bullet_position);
    bullet_transform.set_world_rotation(gun_rotation);

    // set velocity
    let speed = 4.0;
    let velocity = speed * forward;
    bullet_element.physics().set_world_velocity(velocity);
}
/*
#[derive(Clap)]
struct Opts {
    #[clap(short, long)]
    position: Vec<f32>,

    #[clap(short, long)]
    world: bool,

    #[clap(short, long, conflicts_with("world"))]
    local: bool,
}

enum Coordinate {
    Local,
    World,
}

struct Config {
    coordinate: Coordinate,
    position: Option<Vector3>,
}

static CONFIG: Lazy<Config> = Lazy::new(|| parse_args());

fn parse_args() -> Config {
    let opts = Opts::parse();

    let position = if let [x, y, z] = opts.position[..] {
        Some(Vector3::new(x, y, z))
    } else {
        None
    };

    let coordinate = if opts.world {
        Coordinate::Local
    } else {
        Coordinate::World
    };

    Config {
        coordinate: coordinate,
        position: position,
    }
}

#[no_mangle]
fn update() {
    let transform = Transform::myself();

    if let Some(position) = &CONFIG.position {
        match &CONFIG.coordinate {
            Coordinate::Local => transform.set_local_position(*position),
            Coordinate::World => transform.set_world_position(*position),
        }
    }
}
*/
