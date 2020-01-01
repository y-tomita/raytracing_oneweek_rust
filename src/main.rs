extern crate chapter;
use std::env;

use chapter::ch1::*;
use chapter::ch3::*;
use chapter::ch4::*;
use chapter::ch5::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let nx = args[1].parse::<i32>().unwrap();
    let ny = args[2].parse::<i32>().unwrap();
    let number = args[3].parse::<i32>().unwrap();
    match number
    {
        1 => ch1_first_draw(nx, ny),
        3 => ch3_ray_simple_camera_backgrounds(nx, ny),
        4 => ch4_add_sphere(nx, ny),
        5 => ch5_surfase_normals_and_multiple_objects(nx, ny),
        _ => println!("no implement"),
    }
}
