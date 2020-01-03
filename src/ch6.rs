extern crate math;
extern crate hitable;

use std::f64;

use math::vec::*;
use math::ray::*;
use math::camera::*;
use hitable::hitable_trait::*;
use hitable::sphere::*;

use super::ppm_util::*;

/// impl antialiasing
pub fn ch6_antialiasing(nx: i32, ny: i32)
{
    let draw_obj = ScreenObjects{
        components: vec![
            Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)),
            Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)),
        ],
    };
    let cam = Camera::new();
    let ns = 100;

    ppm_print_header(nx, ny);
    for y in (0..ny).rev()
    {
        for x in 0..nx
        {
            let mut col_vec = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..ns
            {
                let rand1 = math::drand48();
                let rand2 = math::drand48();

                let u = ((x as f64) + rand1) / (nx as f64);
                let v = ((y as f64) + rand2) / (ny as f64);

                let r = cam.get_ray(u, v);
                let col = color(r, &draw_obj);
                
                col_vec = col_vec + col;
            }
            col_vec = col_vec / (ns as f64);
            ppm_print_rgb(col_vec.r(), col_vec.g(), col_vec.b());
        }
    }
}

/// create color from Ray
fn color(r: Ray, draw_obj: &ScreenObjects) -> Vec3
{
    let mut rec = HitRecord{
        t: 0.0,
        p: Vec3::new(0.0, 0.0, 0.0),
        normal: Vec3::new(0.0, 0.0, 0.0),
    };
    
    // if hit anything, return normal map
    if draw_obj.is_hit_anything(r, 0.0, f64::MAX, &mut rec)
    {
        return 0.5 * Vec3::new(rec.normal.x + 1.0, rec.normal.y + 1.0, rec.normal.z + 1.0);
    }

    // if ray doesn't hit the sphere, paint background
    let unit_direction = r.direction.make_unit_vec();
    let t = 0.5 * (unit_direction.y + 1.0);
    let a = (1.0 - t) * Vec3::new(1.0, 1.0, 1.0);
    let tb = t * Vec3::new(0.5, 0.7, 1.0);
    
    a + tb
}