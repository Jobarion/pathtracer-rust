use crate::geometry::intersection::Intersection;
use crate::geometry::ray::Ray;
use super::material::Material;

pub struct GlossyMaterial {
    glossiness: f64,
    base: Box<Material>
}

impl GlossyMaterial {
    pub fn new(glossiness: f64, base: Box<Material>) -> GlossyMaterial {
        GlossyMaterial { glossiness, base }
    }
}

impl Material for GlossyMaterial {
    fn get_next_ray<'a>(&self, incoming: Ray, intersection: Intersection) -> Ray {
        let reflection = incoming.direction.reflect(&intersection.normal);
        let mut ray = self.base.get_next_ray(incoming, intersection);
        let direction = ray.direction.scale(self.glossiness).add(&reflection.scale(1.0 - self.glossiness)).normalize();
        ray.direction = direction;
        ray.strength = ray.strength * (1.0 - self.glossiness) + self.glossiness;//TODO this looks wrong
        return ray;
    }
}