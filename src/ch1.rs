
use crate::math::vec::{Vec3};
use crate::ppm_util::*;

pub fn ch1_first_draw(nx: i32, ny: i32)
{
    ppm_print_header(nx, ny);
    for y in (0..ny).rev()
    {
        for x in 0..nx
        {
            let color = Vec3 {
                x: (x as f64) / (nx as f64),
                y: (y as f64) / (ny as f64),
                z: 0.2,
            };            
            ppm_print_rgb(color.r(), color.g(), color.b());
        }
    }
}