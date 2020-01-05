use std::f64;

extern crate math;
extern crate hitable;

use std::rc::Rc;
use math::vec::*;
use math::ray::*;
use hitable::hitable_trait::*;
use hitable::sphere::*;
use hitable::material::*;

use super::ppm_util::*;

/// create simple sphere image
pub fn ch05_surfase_normals_and_multiple_objects(nx: i32, ny: i32)
{
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    let draw_obj = ScreenObjects{
        components: vec![
            Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, Rc::new(Lambertian::new(Vec3::zero())))),
            Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, Rc::new(Lambertian::new(Vec3::zero())))),
        ],
    };

    ppm_print_header(nx, ny);
    for y in (0..ny).rev()
    {
        for x in 0..nx
        {
            let u = (x as f64) / (nx as f64);
            let v = (y as f64) / (ny as f64);
            let direction = lower_left_corner + u * horizontal + v * vertical;
            let r = Ray::new(origin, direction);
            let col = color(r, &draw_obj);

            ppm_print_rgb(col.r(), col.g(), col.b());
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
        mat: Rc::new(Lambertian::new(Vec3::new(0.0, 0.0, 0.0))),
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