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
        for component in self.components.iter()
        {
           if component.hit(r, t_min, t_max, hit_record)
           {
               return true;
           }
        }

        false
    }
}