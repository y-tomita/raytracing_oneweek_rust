extern crate math;

use std::rc::Rc;

use math::vec::*;
use math::ray::Ray;

use super::hitable_trait::*;
use super::material::*;

pub struct Sphere
{
    pub center: Vec3,
    pub radius: f64,

    pub mat: Rc::<dyn Material>,
}

impl Sphere
{
    pub fn new(center: Vec3, radius: f64, material: Rc::<dyn Material>) -> Sphere
    {
        Sphere { 
            center: center,
            radius: radius,
            mat: material,
        }
    }

    fn update_record_point(&self, rec: &mut HitRecord, ray: &Ray, discriminant: f64)
    {
        rec.t = discriminant;
        rec.p = ray.point_at_parameter(discriminant);
        rec.normal = (rec.p - self.center) / self.radius;
        rec.mat = Rc::clone(&self.mat);
    }
}

impl Hitable for Sphere
{
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool
    {
        let oc = r.origin - self.center;
        let a = dot(r.direction, r.direction);
        let b = dot(oc, r.direction);
        let c = dot(oc, oc) - (self.radius * self.radius);

        let discriminant = b*b - a*c;
        let discriminant_sq = discriminant.sqrt();
        if discriminant > 0.0
        {
            let temp = (-b - discriminant_sq) / a;
            if t_min < temp && temp < t_max
            {
                self.update_record_point(rec, &r, temp);
                return true;
            }

            let temp = (-b + discriminant_sq) / a;
            if t_min < temp && temp < t_max
            {
                self.update_record_point(rec, &r, temp);
                return true;
            }
        }

        false
    }
}