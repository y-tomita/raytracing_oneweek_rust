use crate::math::vec::*;
use crate::math::ray::Ray;

pub struct HitRecord
{
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
}

pub trait Hitable
{
    pub fn hit(r: Ray, t_min: f64, t_max: f64, rec: &HitRecord) -> bool;
}