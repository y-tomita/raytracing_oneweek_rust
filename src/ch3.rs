use crate::math::vec::{Vec3};
use crate::math::ray::{Ray};

use crate::ppm_util::*;

pub fn ch3_ray_simple_camera_backgrounds(nx: i32, ny: i32)
{
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);
    
    ppm_print_header(nx, ny);
    for y in (0..ny).rev()
    {
        for x in 0..nx
        {
            let u = (x as f64) / (nx as f64);
            let v = (y as f64) / (ny as f64);
            let direction = lower_left_corner + u * horizontal + v * vertical;
            let r = Ray::new(origin, direction);
            let col = color(r);

            ppm_print_rgb(col.r(), col.g(), col.b());
        }
    }
}

fn color(r: Ray) -> Vec3
{
    let unit_direction = r.direction.make_unit_vec();
    let t = 0.5 * (unit_direction.y + 1.0);
    let a = (1.0 - t) * Vec3::new(1.0, 1.0, 1.0);
    let tb = t * Vec3::new(0.5, 0.7, 1.0);
    
    a + tb
}