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

impl Dielectric
{
    pub fn new(ref_idx: f64) -> Dielectric
    {
        Dielectric {
            ref_idx: ref_idx,
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
    fn scatter(&self, r_in: Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool
    {
        let reflected = reflect(r_in.direction, rec.normal);
        *attenuation = Vec3::new(1.0, 1.0, 1.0);

        let mut ni_over_nt = self.ref_idx;
        let mut outward_normal = rec.normal;
        let cosine;
        if dot(r_in.direction, rec.normal) > 0.0
        {
            outward_normal = -rec.normal;
            cosine = self.ref_idx * dot(r_in.direction, rec.normal) / r_in.direction.length();
        }
        else
        {
            ni_over_nt = 1.0 / self.ref_idx;
            cosine = -dot(r_in.direction, rec.normal) / r_in.direction.length();
        }

        let reflect_prob;
        let mut refracted = Vec3::zero();
        if refract(r_in.direction, outward_normal, ni_over_nt, &mut refracted)
        {
            reflect_prob = schlick(cosine, self.ref_idx);
        }
        else
        {
            *scattered = Ray::new(rec.p, reflected);
            reflect_prob = 1.0;
        }

        if math::drand48() < reflect_prob
        {
            *scattered = Ray::new(rec.p, reflected);
        }
        else
        {
            *scattered = Ray::new(rec.p, refracted);
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
        return true;
    }

    false
}

fn schlick(cosine: f64, ref_idx: f64) -> f64
{
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}