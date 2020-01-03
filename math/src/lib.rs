extern crate rand;

pub mod ray;
pub mod vec;
pub mod camera;

use rand::Rng;

pub fn drand48() -> f64
{
    rand::thread_rng().gen_range(0.0, 1.0)
}