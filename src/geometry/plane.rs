use std::ptr::addr_of;
use crate::geometry::intersection::Intersection;
use crate::geometry::ray::Ray;
use crate::geometry::surface::Surface;
use crate::geometry::vec3::Vec3;

pub struct Plane {
    normal: Vec3,
    position: Vec3
}

impl Plane {
    pub fn new(position: Vec3, normal: Vec3) -> Plane {
        Plane { normal, position }
    }
}

impl Surface for Plane {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let origin = ray.position.subtract(&self.position);
        let d = self.normal.dot(&ray.direction);
        if d == 0.0 {
            return None;
        }
        let t = -self.normal.dot(&origin) / d;
        if t <= 0.0 {
            return None;
        }
        let pos = ray.position.add(&ray.direction.scale(t));
        let normal = if d >= 0.0 {
            self.normal.scale(-1.0)
        } else {
            self.normal.clone()
        };
        Some(Intersection::new(pos, normal, Vec3::new(0.0, 0.0, 0.0), t))
    }
}