use crate::geometry::intersection::Intersection;
use super::vec3::Vec3;
use super::surface::Surface;
use super::volume::Volume;
use super::ray::Ray;

pub struct Sphere {
    pub position: Vec3,
    pub radius: f64,
    radius_squared: f64
}

impl Sphere {
    pub fn new(position: Vec3, radius: f64) -> Sphere {
        Sphere { position, radius, radius_squared: radius * radius }
    }
}

impl Surface for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let a = ray.direction.length_squared();
        let c_offset = ray.position.subtract(&self.position);
        let b = 2.0 * ray.direction.dot(&c_offset);
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

        let pos = ray.position.add(&ray.direction.scale(t));
        let normal = pos.subtract(&self.position).normalize();
        let tangent = Vec3::new(0.0, 1.0, 0.0).cross(&normal).normalize();
        let inter = Intersection::new(pos, normal, tangent, t);

        Option::Some(inter)
    }
}

impl Volume for Sphere {
    fn is_inside(&self, vec: &Vec3) -> bool {
        vec.subtract(&self.position).length_squared() < self.radius_squared
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::ray::Ray;
    use crate::geometry::sphere::Sphere;
    use crate::geometry::surface::Surface;
    use crate::geometry::vec3::Vec3;

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