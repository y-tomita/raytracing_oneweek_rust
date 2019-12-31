use crate::math::vec::*;
use crate::math::ray::{Ray};
use crate::ppm_util::*;

/// create simple sphere image
pub fn ch5_surfase_normals_and_multiple_objects(nx: i32, ny: i32)
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

/// create color from Ray
fn color(r: Ray) -> Vec3
{
    // if ray hits the sphere, paint to screen
    let t = hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0
    {
        let normal = make_unit(r.point_at_parameter(t) - Vec3::new(0.0, 0.0, -1.0));
        return 0.5 * Vec3::new(normal.x + 1.0, normal.y + 1.0, normal.z + 1.0);
    }

    // if ray doesn't hit the sphere, paint background
    let unit_direction = r.direction.make_unit_vec();
    let t = 0.5 * (unit_direction.y + 1.0);
    let a = (1.0 - t) * Vec3::new(1.0, 1.0, 1.0);
    let tb = t * Vec3::new(0.5, 0.7, 1.0);
    
    a + tb
}

/// judge if hit sphere
fn hit_sphere(center: Vec3, radius: f64, r: Ray) -> f64
{
    let oc = r.origin - center;
    let a = r.direction.dot(r.direction);
    let b = 2.0 * dot(r.direction, oc);
    let c = dot(oc, oc) - (radius * radius);

    // b^2 - 4ac > 0
    let discriminant = b*b - 4.0 * a * c;

    if discriminant < 0.0
    {
        return -1.0;
    }
    else
    {
        // (-b - √b^2 -4ac) / 2a
        let denominator = 2.0 * a;
        let numerator = -b - discriminant.sqrt();

        return numerator / denominator;
    }
}