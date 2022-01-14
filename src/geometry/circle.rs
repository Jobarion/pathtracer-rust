use bvh::aabb::{AABB, Bounded};
use bvh::bounding_hierarchy::BHShape;
use glam::Vec3;
use crate::geometry::intersection::Intersection;
use crate::geometry::ray::Ray;
use crate::geometry::surface::Surface;

pub struct Circle {
    position: Vec3,
    normal: Vec3,
    radius_squared: f32
}

impl Circle {
    pub fn new(position: Vec3, normal: Vec3, radius: f32) -> Circle {
        Circle { normal, position, radius_squared: radius * radius }
    }
}

impl Bounded for Circle {
    fn aabb(&self) -> AABB {
        todo!()
    }
}

impl BHShape for Circle {
    fn set_bh_node_index(&mut self, _: usize) {
        todo!()
    }

    fn bh_node_index(&self) -> usize {
        todo!()
    }
}

impl Surface for Circle {

    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let origin = ray.position - self.position;
        let d = self.normal.dot(ray.direction);
        if d == 0.0 {
            return None;
        }
        let t = -self.normal.dot(origin) / d;
        if t <= 0.0 {
            return None;
        }
        let pos = ray.position + ray.direction * t;
        if pos.distance_squared(self.position) > self.radius_squared {
            return None;
        }
        let normal = if d >= 0.0 {
            self.normal * -1.0
        } else {
            self.normal
        };
        Some(Intersection::new(pos, normal, Vec3::new(0.0, 0.0, 0.0), t * t))
    }
}