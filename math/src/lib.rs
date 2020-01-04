extern crate rand;

pub mod ray;
pub mod vec;
pub mod camera;

use vec::*;
use rand::Rng;

pub fn drand48() -> f64
{
    rand::thread_rng().gen_range(0.0, 1.0)
}

pub fn random_in_unit_sphere() -> Vec3
{
    loop
    {
        let p = 2.0 * Vec3::new(drand48(), drand48(), drand48()) - Vec3::new(1.0, 1.0, 1.0);
        if p.squared_length() >= 1.0
        {
            return p;
        }
    }
}