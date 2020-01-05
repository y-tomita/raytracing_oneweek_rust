use std::env;

mod ch01;
mod ch03;
mod ch04;
mod ch05;
mod ch06;
mod ch07;
mod ch08;
mod ch09;
mod ch10;
mod ppm_util;

use ch01::*;
use ch03::*;
use ch04::*;
use ch05::*;
use ch06::*;
use ch07::*;
use ch08::*;
use ch09::*;
use ch10::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let nx = args[1].parse::<i32>().unwrap();
    let ny = args[2].parse::<i32>().unwrap();
    let number = args[3].parse::<i32>().unwrap();
    match number
    {
        1 => ch01_first_draw(nx, ny),
        3 => ch03_ray_simple_camera_backgrounds(nx, ny),
        4 => ch04_add_sphere(nx, ny),
        5 => ch05_surfase_normals_and_multiple_objects(nx, ny),
        6 => ch06_antialiasing(nx, ny),
        7 => ch07_diffuse_materials(nx, ny),
        8 => ch08_metal(nx, ny),
        9 => ch09_dielectrics(nx, ny),
        10 => ch10_positionable_camera(nx, ny),
        _ => println!("no implement"),
    }
}