use super::intersection::Intersection;
use super::ray::Ray;

pub trait Surface: Sync + Send {

    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
}