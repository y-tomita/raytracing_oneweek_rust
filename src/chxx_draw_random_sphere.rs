extern crate math;
extern crate hitable;

use std::f64;
use std::rc::Rc;

use math::vec::*;
use math::ray::*;
use math::camera::*;

use hitable::hitable_trait::*;
use hitable::sphere::*;
use hitable::material::*;

use super::ppm_util::*;

/// impl random sphere
pub fn chxx_draw_random_sphere(nx: i32, ny: i32)
{
    let mut draw_obj = ScreenObjects{
        components: vec![
            Box::new(Sphere::new(Vec3::new( 0.0, -1000.0, 0.0), 1000.0, Rc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5))))),
            Box::new(Sphere::new(Vec3::new( 0.0, 1.0, 0.0),     1.0,    Rc::new(Dielectric::new(1.5)))),
            Box::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0),     1.0,    Rc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1))))),
            Box::new(Sphere::new(Vec3::new( 4.0, 1.0, 0.0),     1.0,    Rc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)))),
        ],
    };

    for a in -11..11
    {
        let fa = a as f64;
        for b in -11..11
        {
            let fb = b as f64;
            let choose_mat = math::drand48();
            let center = Vec3::new(fa + 0.9 * math::drand48(), 0.2, fb + 0.9 * math::drand48());
            if choose_mat < 0.8
            {   // diffuse
                let r = math::drand48() * math::drand48();
                let g = math::drand48() * math::drand48();
                let b = math::drand48() * math::drand48();
                let albedo = Vec3::new(r, g, b);
                let fuzz = 0.5 * math::drand48();
                draw_obj.add(Box::new(Sphere::new(center, 0.2, Rc::new(Metal::new(albedo, fuzz)))));
            }
            else if choose_mat < 0.95
            {   // metal
                let r = 0.5 * (1.0 + math::drand48());
                let g = 0.5 * (1.0 + math::drand48());
                let b = 0.5 * (1.0 + math::drand48());
                let albedo = Vec3::new(r, g, b);
                draw_obj.add(Box::new(Sphere::new(center, 0.2, Rc::new(Lambertian::new(albedo)))));
            }
            else
            {   // glass
                draw_obj.add(Box::new(Sphere::new(center, 0.2, Rc::new(Dielectric::new(1.5)))));
            }
        }
    }

    let fnx = nx as f64;
    let fny = ny as f64;
    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let dist_to_focus= 10.0;
    let aperture = 0.1;
    let camera = Camera::new_by_lookat_blur(
        lookfrom,
        lookat,
        Vec3::new(0.0,1.0, 0.0),
        20.0,
        fnx / fny,
        aperture,
        dist_to_focus
    );
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

                let u = ((x as f64) + rand1) / fnx;
                let v = ((y as f64) + rand2) / fny;
                let r = camera.get_ray_with_blur(u, v);
                let col = color(r, &draw_obj, 0);
                
                col_vec = col_vec + col;
            }
            col_vec = col_vec / (ns as f64);
            col_vec = convert_to_gamma(col_vec);
            ppm_print_rgb(col_vec.r(), col_vec.g(), col_vec.b());
        }
    }
}

/// create color from Ray
fn color(r: Ray, draw_obj: &ScreenObjects, depth: i32) -> Vec3
{
    let mut rec = HitRecord{
        t: 0.0,
        p: Vec3::new(0.0, 0.0, 0.0),
        normal: Vec3::new(0.0, 0.0, 0.0),
        mat: Rc::new(Lambertian::new(Vec3::new(1.0, 1.0, 1.0))),
    };

    if draw_obj.is_hit_anything(r, 0.001, f64::MAX, &mut rec)
    {
        let mut scatterd = Ray::new(Vec3::zero(), Vec3::zero());
        let mut attenuation = Vec3::zero();
        if depth < 50 && rec.mat.scatter(r, &rec, &mut attenuation, &mut scatterd)
        {
            return attenuation * color(scatterd, draw_obj, depth + 1);
        }
        else
        {
            return Vec3::zero();
        }
    }

    // if ray doesn't hit the sphere, paint background
    let unit_direction = r.direction.make_unit_vec();
    let t = 0.5 * (unit_direction.y + 1.0);
    let a = (1.0 - t) * Vec3::new(1.0, 1.0, 1.0);
    let tb = t * Vec3::new(0.5, 0.7, 1.0);
    
    a + tb
}