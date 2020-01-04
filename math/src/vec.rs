use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;

#[derive(Clone, Copy, Debug)]
pub struct Vec3
{
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Add<Vec3> for Vec3
{
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3
    {
        Vec3 { 
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub<Vec3> for Vec3
{
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3
    {
        Vec3 { 
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<Vec3> for Vec3
{
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3
    {
        Vec3 { 
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<Vec3> for f64
{
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3
    {
        Vec3 { 
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Div<f64> for Vec3
{
    type Output = Vec3;

    fn div(self, other: f64) -> Vec3
    {
        Vec3 { 
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl Div<Vec3> for Vec3
{
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3
    {
        Vec3 { 
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl Vec3
{
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 
    {
        Vec3 { x: x, y: y, z:z }
    }

    pub fn zero() -> Vec3
    {
        Vec3::new(0.0, 0.0, 0.0)
    }

    pub fn r(&self) -> f64 { self.x }
    
    pub fn g(&self) -> f64 { self.y }
    
    pub fn b(&self) -> f64 { self.z }

    pub fn squared_length(&self) -> f64
    {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64
    {
        self.squared_length().sqrt()
    }

    pub fn make_unit_vec(&self) -> Self
    {
        let len = self.length();
        Self {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }

    pub fn dot(&self, other: Vec3) -> f64
    {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: Vec3) -> Vec3
    {
        // cx = aybz − azby
        // cy = azbx − axbz
        // cz = axby − aybx
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

pub fn dot(a: Vec3, b: Vec3) -> f64
{
    a.dot(b)
}

pub fn make_unit(a: Vec3) -> Vec3
{
    a.make_unit_vec()
}

pub fn convert_to_gamma(col: Vec3) -> Vec3
{
    Vec3::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt())
}