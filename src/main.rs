mod math;
mod ppm_util;

mod ch1;
mod ch3;
mod ch4;

use ch1::*;
use ch3::*;
use ch4::*;

fn main() {
    let nx = 200;
    let ny = 100;
    let number = 4;
    match number
    {   // simple rgb gradation
        1 => ch1_first_draw(nx, ny),
        3 => ch3_ray_simple_camera_backgrounds(nx, ny),
        4 => ch4_add_sphere(nx, ny),
        _ => println!("no implement"),
    }
}
