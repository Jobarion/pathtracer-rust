use super::super::geometry::intersection::Intersection;
use super::super::geometry::ray::Ray;

pub trait Material: Sync + Send {
    fn get_next_ray(&self, incoming: Ray, intersection: Intersection) -> Ray;
}

pub trait Radiator: Sync + Send {
    fn get_intensity(&self, wavelength: f32) -> f32;
}