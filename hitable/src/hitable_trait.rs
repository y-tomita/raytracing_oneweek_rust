extern crate math;

use std::rc::Rc;
use math::vec::*;
use math::ray::Ray;

use super::material::*;

pub struct HitRecord
{
    /// ray parameter t
    pub t: f64,
    /// ray hit point
    pub p: Vec3,
    /// normal vec from hit point
    pub normal: Vec3,
    /// material
    pub mat: Rc<dyn Material>,
}

pub trait Hitable
{
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

pub struct ScreenObjects
{
    pub components: Vec<Box<dyn Hitable>>,
}

impl ScreenObjects
{
    pub fn is_hit_anything(&self, r: Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool
    {
        let mut closest_so_far = t_max;
        let mut anything_hit = false;
        for component in self.components.iter()
        {
            let mut temp_hit = HitRecord {
                t: 0.0,
                p: Vec3::zero(),
                normal: Vec3::zero(),
                mat: Rc::new(Lambertian::new(Vec3::zero())),
            };
            if component.hit(r, t_min, closest_so_far, &mut temp_hit)
            {
                *hit_record = temp_hit;
                closest_so_far = hit_record.t;
                anything_hit = true;
            }
        }

        anything_hit
    }

    pub fn add(&mut self, component: Box<dyn Hitable>)
    {
        self.components.push(component);
    }
}