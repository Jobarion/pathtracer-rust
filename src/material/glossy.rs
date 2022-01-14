use crate::geometry::intersection::Intersection;
use crate::geometry::ray::Ray;
use super::material::Material;
use super::super::geometry::util;

pub struct GlossyMaterial {
    glossiness: f32,
    base: Box<dyn Material>
}

impl GlossyMaterial {
    pub fn new(glossiness: f32, base: Box<dyn Material>) -> GlossyMaterial {
        GlossyMaterial { glossiness, base }
    }
}

impl Material for GlossyMaterial {
    fn get_next_ray<'a>(&self, incoming: Ray, intersection: Intersection) -> Ray {
        let reflection = util::reflect(incoming.direction, intersection.normal);
        let mut ray = self.base.get_next_ray(incoming, intersection);
        let direction = (ray.direction * self.glossiness + reflection * (1.0 - self.glossiness)).normalize();
        ray.direction = direction;
        ray.strength = ray.strength * (1.0 - self.glossiness) + self.glossiness;//TODO this looks wrong
        return ray;
    }
}