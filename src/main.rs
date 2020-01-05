use std::env;

mod ch1;
mod ch3;
mod ch4;
mod ch5;
mod ch6;
mod ch7;
mod ch8;
mod ch9;
mod ch10;
mod ppm_util;

use ch1::*;
use ch3::*;
use ch4::*;
use ch5::*;
use ch6::*;
use ch7::*;
use ch8::*;
use ch9::*;
use ch10::*;

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
        6 => ch6_antialiasing(nx, ny),
        7 => ch7_diffuse_materials(nx, ny),
        8 => ch8_metal(nx, ny),
        9 => ch9_dielectrics(nx, ny),
        10 => ch10_positionable_camera(nx, ny),
        _ => println!("no implement"),
    }
}