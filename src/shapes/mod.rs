use crate::{utility::linalg::{Ray3, Point3}, config::Float};

use self::transform::Transform;

pub mod sphere;
pub mod transform;

#[derive(Debug)]
pub struct IntersectionInfo {
    pub did_hit: bool,
    pub point: Point3,
    pub t: Float,
}

impl Default for IntersectionInfo {
    fn default() -> Self {
        Self {
            did_hit: false,
            point: Point3::default(),
            t: Float::INFINITY,
        }
    }
}

impl IntersectionInfo {
    pub fn no_intersection() -> Self {
        Self {
            did_hit: false,
            ..Default::default()
        }
    }
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray3) -> IntersectionInfo;
}

/// Rusty idiom for indicating that an implementor really should be keeping track of
/// a transform internally.
pub trait Transformable {
    fn get_transform(&self) -> Transform;
}

pub trait SurfaceLike: Intersectable + Transformable {}
