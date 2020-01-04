extern crate math;

use math::vec::*;
use math::ray::Ray;

use super::hitable_trait::*;

pub trait Material
{
    fn scatter(&self, r_in: Ray, hit_record: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool;
}

pub struct Lambertian
{
    pub albedo: Vec3,
}

pub struct Metal
{
    pub albedo: Vec3,
}

impl Lambertian
{
    pub fn new(albedo: Vec3) -> Lambertian
    {
        Lambertian {
            albedo: albedo,
        }
    }
}

impl Metal
{
    pub fn new(albedo: Vec3) -> Metal
    {
        Metal {
            albedo: albedo,
        }
    }
}

impl Material for Lambertian
{
    fn scatter(&self, _r_in: Ray, hit_record: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool
    {
        let target = hit_record.p + hit_record.normal + math::random_in_unit_sphere();
        *scattered = Ray::new(hit_record.p, target - hit_record.p);
        *attenuation = self.albedo;
        true
    }
}

impl Material for Metal
{
    fn scatter(&self, r_in: Ray, hit_record: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool
    {
        let reflected = reflect(r_in.direction.make_unit_vec(), hit_record.normal);
        *scattered = Ray::new(hit_record.p, reflected);
        *attenuation = self.albedo;
        
        dot(scattered.direction, hit_record.normal) > 0.0
    }   
}

fn reflect(v: Vec3, n: Vec3) -> Vec3
{
    v - 2.0 * dot(v, n) * n
}