use crate::math::vec::*;
use crate::math::ray::Ray;

pub struct HitRecord
{
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
}

pub trait Hitable
{
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

pub struct ScreenObjects
{
    pub components: Vec<Box<dyn Hitable>>,
}

// impl ScreenObjects
// {
//     pub fn run(&self, r: Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool
//     {
//         for component in self.components.iter()
//         {
//            if component.hit(r, t_min, t_max, &mut hit_record)
//            {
//                return true;
//            }
//         }

//         false
//     }
// }