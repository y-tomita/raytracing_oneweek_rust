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
    pub fuzz: f64,
}

pub struct Dielectric
{
    pub ref_idx: f64,
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
    pub fn new(albedo: Vec3, fuzz: f64) -> Metal
    {
        Metal {
            albedo: albedo,
            fuzz: fuzz,
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
        *scattered = Ray::new(hit_record.p, reflected + self.fuzz * math::random_in_unit_sphere());
        *attenuation = self.albedo;
        
        dot(scattered.direction, hit_record.normal) > 0.0
    }   
}

impl Material for Dielectric
{
    fn scatter(&self, r_in: Ray, hit_record: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool
    {
        let reflected = reflect(r_in.direction, hit_record.normal);
        *attenuation = Vec3::new(1.0, 1.0, 0.0);

        let mut ni_over_nt = self.ref_idx;
        let mut outward_normal = hit_record.normal;
        if dot(r_in.direction, hit_record.normal) > 0.0
        {
            outward_normal = -hit_record.normal;
        }
        else
        {
            ni_over_nt = 1.0 / self.ref_idx;
        }

        let mut refracted = Vec3::zero();
        if refract(r_in.direction, outward_normal, ni_over_nt, &mut refracted)
        {
            *scattered = Ray::new(hit_record.p, refracted);
        }
        else
        {
            *scattered = Ray::new(hit_record.p, reflected);
            return false;
        }

        true
    }
}

fn reflect(v: Vec3, n: Vec3) -> Vec3
{
    v - 2.0 * dot(v, n) * n
}

fn refract(v: Vec3, n: Vec3, ni_over_nt: f64, refracted: &mut Vec3) ->  bool
{
    let uv = v.make_unit_vec();
    let dt = dot(uv, n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * ( 1.0 - dt * dt);
    if discriminant > 0.0
    {
        *refracted = ni_over_nt * (uv - dt * n) - discriminant.sqrt() * n;
    }

    false
}