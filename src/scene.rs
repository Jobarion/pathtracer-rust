use crate::entity::Entity;
use crate::geometry::intersection::Intersection;
use crate::geometry::quaternion::Quaternion;
use crate::geometry::ray::Ray;
use crate::geometry::vec3::Vec3;
use crate::ptrandom;

pub struct Scene {
    entities: Vec<Entity>,
    pub camera: Camera,
}

impl Scene {

    pub fn new(entities: Vec<Entity>, camera: Camera) -> Scene {
        Scene { entities, camera }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<(&Entity, Intersection)> {
        let mut min_distance = f64::INFINITY;
        let mut result: Option<(&Entity, Intersection)> = None;
        for e in &self.entities {
            let surface = match e {
                Entity::DARK(s, _) => s,
                Entity::LUMINOUS(s, _) => s
            };
            let intersection = surface.intersect(ray);
            if let Some(i) = intersection {
                let dist = i.distance;
                if dist < min_distance {
                    result = Some((&e, i));
                    min_distance = dist;
                }
            }
        }
        return result;
    }
}

pub struct Camera {
    pub position: Vec3,
    pub orientation: Quaternion,
    pub field_of_view: f64,
    pub focal_distance: f64,
    pub depth_of_field: f64,
    pub chromatic_aberration: f64
}

impl Camera {

    pub fn new(position: Vec3,
               orientation: Quaternion,
               field_of_view: f64,
               focal_distance: f64,
               depth_of_field: f64,
               chromatic_aberration: f64) -> Camera {
        Camera { position, orientation, field_of_view, focal_distance, depth_of_field, chromatic_aberration }
    }

    fn get_screen_ray(&self, x: f64, y: f64, chroma_factor: f64, dof_angle: f64, dof_radius: f64) -> Ray {
        let screen_distance = 1.0 / (self.field_of_view * 0.5).tan();
        let xy = x * chroma_factor;
        let ys = y * chroma_factor;

        let direction = Vec3::new(xy, screen_distance, -ys).normalize();
        let focus_point = direction.scale(self.focal_distance / direction.y);
        let lens_point = Vec3::new(dof_angle.cos() * dof_radius, 0.0, dof_angle.sin() * dof_radius);
        Ray::new(
            self.position.add(&lens_point.rotate(&self.orientation)),
            focus_point.subtract(&lens_point).rotate(&self.orientation).normalize(),
            0.0,
            1.0
        )
    }

    pub fn get_ray(&self, x: f64, y: f64, wavelength: f64) -> Ray {
        let dof_angle = ptrandom::get_longitude();
        let dof_radius = ptrandom::get_unit() / self.depth_of_field;
        let d = (wavelength - 580.0) / 200.0;
        let chroma_zoom = 1.0 + d * self.chromatic_aberration;
        let mut ray = self.get_screen_ray(x, y, chroma_zoom, dof_angle, dof_radius);
        ray.wavelength = wavelength;
        return ray;
    }
}