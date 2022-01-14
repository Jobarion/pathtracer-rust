use bvh::aabb::{AABB, Bounded};
use bvh::bounding_hierarchy::BHShape;
use crate::geometry::intersection::Intersection;
use glam::Vec3;
use super::surface::Surface;
use super::ray::Ray;

pub struct Sphere {
    pub position: Vec3,
    pub radius: f32,
    radius_squared: f32
}

impl Sphere {
    pub fn new(position: Vec3, radius: f32) -> Sphere {
        Sphere { position, radius, radius_squared: radius * radius }
    }
}

impl Bounded for Sphere {
    fn aabb(&self) -> AABB {
        todo!()
    }
}

impl BHShape for Sphere {
    fn set_bh_node_index(&mut self, _: usize) {
        todo!()
    }

    fn bh_node_index(&self) -> usize {
        todo!()
    }
}

impl Surface for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let a = ray.direction.length_squared();
        let c_offset = ray.position - self.position;
        let b = 2.0 * ray.direction.dot(c_offset);
        let c = &c_offset.length_squared() - self.radius_squared;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return Option::None;
        }

        let d = discriminant.sqrt();
        let t1 = 0.5 * (-b + d) / a;
        let t2 = 0.5 * (-b - d) / a;

        let t = t1.min(t2);

        if t <= 0.0 {
            return Option::None;
        }

        let pos = ray.position + ray.direction * t;
        let normal = (pos - self.position).normalize();
        let tangent = Vec3::new(0.0, 1.0, 0.0).cross(normal).normalize();
        let inter = Intersection::new(pos, normal, tangent, t * t);

        Option::Some(inter)
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::ray::Ray;
    use crate::geometry::sphere::Sphere;
    use crate::geometry::surface::Surface;
    use glam::Vec3;

    #[test]
    fn sphere_intersection_center_pos_y() {
        let s = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 1.0);
        let r = Ray::new(Vec3::new(0.0, -5.0, 0.0), Vec3::new(0.0, 1.0, 0.0), 1.0, 1.0);
        let i = s.intersect(&r);
        match i {
            None => assert!(false),
            Some(hit) => assert_eq!(hit.position, Vec3::new(0.0, -1.0, 0.0))
        }
    }
    #[test]
    fn sphere_intersection_center_neg_y() {
        let s = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 1.0);
        let r = Ray::new(Vec3::new(0.0, 5.0, 0.0), Vec3::new(0.0, -1.0, 0.0), 1.0, 1.0);
        let i = s.intersect(&r);
        match i {
            None => assert!(false),
            Some(hit) => assert_eq!(hit.position, Vec3::new(0.0, 1.0, 0.0))
        }
    }
    #[test]
    fn sphere_intersection_center_pos_x() {
        let s = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 1.0);
        let r = Ray::new(Vec3::new(-5.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0), 1.0, 1.0);
        let i = s.intersect(&r);
        match i {
            None => assert!(false),
            Some(hit) => assert_eq!(hit.position, Vec3::new(-1.0, 0.0, 0.0))
        }
    }
    #[test]
    fn sphere_intersection_center_neg_x() {
        let s = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 1.0);
        let r = Ray::new(Vec3::new(5.0, 0.0, 0.0), Vec3::new(-1.0, 0.0, 0.0), 1.0, 1.0);
        let i = s.intersect(&r);
        match i {
            None => assert!(false),
            Some(hit) => assert_eq!(hit.position, Vec3::new(1.0, 0.0, 0.0))
        }
    }
    #[test]
    fn sphere_intersection_center_pos_z() {
        let s = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 1.0);
        let r = Ray::new(Vec3::new(0.0, 0.0, -5.0), Vec3::new(0.0, 0.0, 1.0), 1.0, 1.0);
        let i = s.intersect(&r);
        match i {
            None => assert!(false),
            Some(hit) => assert_eq!(hit.position, Vec3::new(0.0, 0.0, -1.0))
        }
    }
    #[test]
    fn sphere_intersection_center_neg_z() {
        let s = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 1.0);
        let r = Ray::new(Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, -1.0), 1.0, 1.0);
        let i = s.intersect(&r);
        match i {
            None => assert!(false),
            Some(hit) => assert_eq!(hit.position, Vec3::new(0.0, 0.0, 1.0))
        }
    }
    #[test]
    fn sphere_intersection_center_large_radius() {
        let s = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 1000.0);
        let r = Ray::new(Vec3::new(-10000.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0), 1.0, 1.0);
        let i = s.intersect(&r);
        match i {
            None => assert!(false),
            Some(hit) => assert_eq!(hit.position, Vec3::new(-1000.0, 0.0, 0.0))
        }
    }
    #[test]
    fn sphere_miss_x_axis() {
        let s = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 10.0);
        let r = Ray::new(Vec3::new(10.1, -5.0, 0.0), Vec3::new(0.0, 1.0, 0.0), 1.0, 1.0);
        let i = s.intersect(&r);
        match i {
            None => assert!(true),
            Some(hit) => assert!(false)
        }
    }
}