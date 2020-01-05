use super::vec::*;
use super::ray::*;

pub struct Camera
{
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera
{
    pub fn new() -> Camera
    {
        Camera {
            lower_left_corner:  Vec3::new(-2.0, -1.0, -1.0),
            horizontal:         Vec3::new(4.0, 0.0, 0.0),
            vertical:           Vec3::new(0.0, 2.0, 0.0),
            origin:             Vec3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn new_by_fov(vfov: f64, aspect: f64) -> Camera
    {
        let theta = vfov * std::f64::consts::PI / 180.0;
        let half_height = (theta * 0.5).tan();
        let half_width = aspect * half_height;

        Camera {
            lower_left_corner:  Vec3::new(-half_width, -half_height, -1.0),
            horizontal:         Vec3::new(2.0 * half_width, 0.0, 0.0),
            vertical:           Vec3::new(0.0, 2.0 * half_height, 0.0),
            origin:             Vec3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn new_by_lookat(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f64, aspect: f64) -> Camera
    {
        let theta = vfov * std::f64::consts::PI / 180.0;
        let half_height = (theta * 0.5).tan();
        let half_width = aspect * half_height;

        let w = (lookfrom - lookat).make_unit_vec();
        let u = cross(vup, w).make_unit_vec();
        let v = cross(w, u);

        Camera {
            lower_left_corner:  lookfrom - half_width * u - half_height * v - w,
            horizontal:         2.0 * half_width * u,
            vertical:           2.0 * half_height * v,
            origin:             lookfrom,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray
    {
        Ray::new(self.origin, self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin)
    }
}