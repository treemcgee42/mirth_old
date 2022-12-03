//! Linear algebra stuff like vectors and matrices

use std::ops;

use crate::config::{Float, FLOAT_ERR, SignCheckable};
use image;
use cgmath::{self, Transform, InnerSpace, SquareMatrix};

pub type Color3 = Vec3;
impl From<Color3> for image::Rgb<f32> {
    fn from(color: Color3) -> Self {
        image::Rgb {
            0:[color.x().into(), color.y().into(), color.z().into()]
        }
    }
}

pub type Point3 = Vec3;

impl Point3 {
    pub fn origin() -> Self {
        Point3::new(0.0, 0.0, 0.0)
    }
}

// S==== VECTOR {{{1

#[derive(Clone, Debug)]
pub struct Vec3 {
    internal: cgmath::Vector3<Float>,
}

impl Vec3 {
    /// Create a new vector with the specified coordinates
    pub fn new(x: Float, y: Float, z: Float) -> Vec3 {
        Vec3 {
            internal: cgmath::Vector3::new(x, y, z),
        }
    }

    /// Retrieve the x coordinate.
    pub fn x(&self) -> Float { self.internal.x }
    /// Retrieve the y coordinate.
    pub fn y(&self) -> Float { self.internal.y }
    /// Retrieve the z coordinate.
    pub fn z(&self) -> Float { self.internal.z }

    /// Change the x coordinate to the specified value.
    pub fn set_x(&mut self, x: Float) { self.internal.x = x; }
    /// Change the y coordinate to the specified value.
    pub fn set_y(&mut self, y: Float) { self.internal.y = y; }
    /// Change the z coordinate to the specified value.
    pub fn set_z(&mut self, z: Float) { self.internal.z = z; }

    pub fn normalize(mut self) -> Self {
        self.internal = self.internal.normalize();
        self
    }

    pub fn normalize_to(mut self, magnitude: Float) -> Self {
        self.internal = self.internal.normalize_to(magnitude);
        self
    }

    pub fn are_equal(v1: &Vec3, v2: &Vec3) -> bool {
        (v1.x() - v2.x()).is_zero()
        && (v1.y() - v2.y()).is_zero()
        && (v1.z() - v2.z()).is_zero()
    }
}

pub fn dot(v1: &Vec3, v2: &Vec3) -> Float {
    cgmath::dot(v1.internal, v2.internal)
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3::new(0.0, 0.0, 0.0)
    }
}

// S==== OPERATOR OVERLOADS {{{2

// Vec3 + Vec3
impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        let v = self.internal + rhs.internal;
        Vec3::new(v.x, v.y, v.z)
    }
}

// Vec3 + &Vec3
impl ops::Add<&Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Self::Output {
        let v = self.internal + rhs.internal;
        Vec3::new(v.x, v.y, v.z)
    }
}

// &Vec3 + Vec3
impl ops::Add<Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        let v = self.internal + rhs.internal;
        Vec3::new(v.x, v.y, v.z)
    }
}

// &Vec3 + &Vec3
impl ops::Add<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Self::Output {
        let v = self.internal + rhs.internal;
        Vec3::new(v.x, v.y, v.z)
    }
}

// Vec3 - Vec3
impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        let v = self.internal - rhs.internal;
        Vec3::new(v.x, v.y, v.z)
    }
}

// Vec3 - &Vec3
impl ops::Sub<&Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Self::Output {
        let v = self.internal - rhs.internal;
        Vec3::new(v.x, v.y, v.z)
    }
}

// &Vec3 - Vec3
impl ops::Sub<Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        let v = self.internal - rhs.internal;
        Vec3::new(v.x, v.y, v.z)
    }
}

// &Vec3 - &Vec3
impl ops::Sub<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Self::Output {
        let v = self.internal - rhs.internal;
        Vec3::new(v.x, v.y, v.z)
    }
}

// Float * Vec3
impl ops::Mul<Vec3> for Float {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self * rhs.x(), self * rhs.y(), self * rhs.z())
    }
}

// Float * &Vec3
impl ops::Mul<&Vec3> for Float {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3::new(self * rhs.x(), self * rhs.y(), self * rhs.z())
    }
}

// E==== OPERATOR OVERLOADS }}}2

// E==== VECTOR }}}1

#[derive(Clone, Debug, Default)]
pub struct Ray3 {
    pub origin: Point3,
    pub direction: Vec3,
    pub min_t: Float,
    pub max_t: Float,
}

impl Ray3 {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self {
            origin,
            direction,
            min_t: FLOAT_ERR,
            max_t: Float::INFINITY,
        }
    }

    pub fn is_in_range(&self, t: Float) -> bool {
        (self.min_t < t) && (t < self.max_t)
    }

    pub fn eval(&self, t: Float) -> Point3 {
        &self.origin + t * &self.direction
    }
}

// S==== MATRIX {{{

#[derive(Clone)]
pub struct Matrix4 {
    internal: cgmath::Matrix4<Float>,
}

impl Default for Matrix4 {
    fn default() -> Self {
        Matrix4 { internal: cgmath::Matrix4::identity() }
    }
}

impl Matrix4 {
    pub fn transform_point(&self, point: &Point3) -> Point3 {
        let internal_point: cgmath::Point3<Float> = cgmath::point3(point.x(), point.y(), point.z());
        let xformed_point = self.internal.transform_point(internal_point);

        Point3::new(xformed_point.x, xformed_point.y, xformed_point.z)
    }

    pub fn transform_vector(&self, vector: &Vec3) -> Vec3 {
        let xformed_vec = self.internal.transform_vector(vector.internal);

        Vec3::new(xformed_vec.x, xformed_vec.y, xformed_vec.z)
    }
}

// E==== MATRIX }}}

