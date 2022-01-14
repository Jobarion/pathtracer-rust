use crate::geometry::util;
use super::super::geometry::intersection::Intersection;
use super::super::geometry::ray::Ray;
use super::material::Material;
use super::super::ptrandom;

pub struct DiffuseGrayMaterial {
    gray_scale: f32
}

impl DiffuseGrayMaterial {

    pub fn new(gray_scale: f32) -> DiffuseGrayMaterial {
        DiffuseGrayMaterial {gray_scale}
    }
}

impl Material for DiffuseGrayMaterial {
    fn get_next_ray<'a>(&self, incoming: Ray, intersection: Intersection) -> Ray {
        let hemi = ptrandom::get_hemisphere_vector();
        let normal = if incoming.direction.dot(intersection.normal) < 0.0 {
            intersection.normal.clone()
        } else {
            intersection.normal * -1.0
        };

        let direction = util::rotate_towards(hemi, normal);
        Ray::new(intersection.position, direction, incoming.wavelength, self.gray_scale)
    }
}

pub struct SimpleDiffuseColoredMaterial {
    wavelength: f32,
    deviation: f32,
    brightness: f32,
}

impl SimpleDiffuseColoredMaterial {
    pub fn new(brightness: f32, wavelength: f32, deviation: f32) -> SimpleDiffuseColoredMaterial {
        SimpleDiffuseColoredMaterial {wavelength, deviation, brightness}
    }
}

impl Material for SimpleDiffuseColoredMaterial {
    fn get_next_ray(&self, incoming: Ray, intersection: Intersection) -> Ray {
        let hemi = ptrandom::get_hemisphere_vector();
        let normal = if incoming.direction.dot(intersection.normal) < 0.0 {
            intersection.normal.clone()
        } else {
            intersection.normal * -1.0
        };

        let direction = util::rotate_towards(hemi, normal);

        let p = (self.wavelength - incoming.wavelength) / self.deviation;
        let q = (-0.5 * p * p).exp();

        Ray::new(intersection.position, direction, incoming.wavelength, self.brightness * q)
    }
}