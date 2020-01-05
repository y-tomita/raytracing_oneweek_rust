use super::vec::*;
use super::ray::*;
use super::drand48;

pub struct Camera
{
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,

    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,

    pub lens_radius: f64,
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

            u: Vec3::zero(),
            v: Vec3::zero(),
            w: Vec3::zero(),

            lens_radius: 0.0,
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

            u: Vec3::zero(),
            v: Vec3::zero(),
            w: Vec3::zero(),

            lens_radius: 0.0,
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

            u: u,
            v: v,
            w: w,

            lens_radius: 0.0,
        }
    }

    pub fn new_by_lookat_blur(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f64, aspect: f64, aperture: f64, focus_dist: f64) -> Camera
    {
        let theta = vfov * std::f64::consts::PI / 180.0;
        let half_height = (theta * 0.5).tan();
        let half_width = aspect * half_height;

        let w = (lookfrom - lookat).make_unit_vec();
        let u = cross(vup, w).make_unit_vec();
        let v = cross(w, u);

        Camera {
            lower_left_corner:  lookfrom - (half_width * focus_dist * u) - (half_height * focus_dist * v) - (focus_dist * w),
            horizontal:         2.0 * half_width * focus_dist * u,
            vertical:           2.0 * half_height * focus_dist * v,
            origin:             lookfrom,

            u: u,
            v: v,
            w: w,

            lens_radius: aperture * 0.5,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray
    {
        Ray::new(self.origin, self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin)
    }

    pub fn get_ray_with_blur(&self, u: f64, v: f64) -> Ray
    {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = rd.x * self.u + rd.y * self.v;
        Ray::new(self.origin + offset, self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset)
    }
}

fn random_in_unit_disk() -> Vec3
{
    loop
    {
        let p = 2.0 * Vec3::new(drand48(), drand48(), 0.0) - Vec3::new(1.0, 1.0, 0.0);
        if dot(p, p) < 1.0
        {
            return p;
        }
    }
}