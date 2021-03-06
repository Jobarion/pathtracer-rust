use bvh::aabb::{AABB, Bounded};
use bvh::bounding_hierarchy::BHShape;
use crate::geometry::intersection::Intersection;
use crate::geometry::ray::Ray;
use crate::geometry::surface::Surface;
use glam::Vec3;

pub struct Plane {
    normal: Vec3,
    position: Vec3
}

impl Plane {
    pub fn new(position: Vec3, normal: Vec3) -> Plane {
        Plane { normal, position }
    }
}

impl Bounded for Plane {
    fn aabb(&self) -> AABB {
        todo!()
    }
}

impl BHShape for Plane {
    fn set_bh_node_index(&mut self, _: usize) {
        todo!()
    }

    fn bh_node_index(&self) -> usize {
        todo!()
    }
}

impl Surface for Plane {
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
        let normal = if d >= 0.0 {
            self.normal * -1.0
        } else {
            self.normal
        };
        Some(Intersection::new(pos, normal, Vec3::new(0.0, 0.0, 0.0), t * t))
    }
}