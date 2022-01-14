use bvh::aabb::Bounded;
use bvh::bounding_hierarchy::BHShape;
use super::intersection::Intersection;
use super::ray::Ray;

pub trait Surface: Sync + Send + Bounded + BHShape {

    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
}