use crate::entity::Entity::{DARK, LUMINOUS};
use crate::geometry::ray::Ray;
use crate::scene::Scene;
use super::ptrandom;

pub struct Photon {
    pub x: f32,
    pub y: f32,
    pub strength: f32,
    pub wavelength: f32
}

pub struct RenderIterator<'a> {
    scene: &'a Scene,
    min_x: f32,
    max_x: f32,
    min_y: f32,
    max_y: f32
}

impl RenderIterator<'_> {
    pub fn new_global(scene: &Scene) -> RenderIterator {
        RenderIterator::new_sliced(scene, -1.0, 1.0, -1.0, 1.0)
    }

    pub fn new_sliced(scene: &Scene, min_x: f32, max_x: f32, min_y: f32, max_y: f32) -> RenderIterator {
        RenderIterator { scene, min_x, max_x, min_y, max_y }
    }

    fn render_slice(&self) -> Photon {
        let wavelength = ptrandom::get_wavelength();
        let x = ptrandom::get_unit() * (self.max_x - self.min_x) + self.min_x;
        let y = ptrandom::get_unit() * (self.max_y - self.min_y) + self.min_y;
        let strength = self.render_camera_ray(x, y, wavelength);
        Photon {x, y, strength, wavelength}
    }

    fn render_camera_ray(&self, x: f32, y: f32, wavelength: f32) -> f32 {
        self.render_ray(self.scene.camera.get_ray(x, y, wavelength))
    }

    fn render_ray(&self, ray: Ray) -> f32 {
        let mut continue_chance = 1.0;
        let mut intensity = 1.0;
        let mut current_ray = ray;
        loop {
            let intersection = self.scene.intersect(&current_ray);
            if let Some(i) = intersection {
                if let LUMINOUS(surface, radiator) = i.0 {
                    return intensity * radiator.get_intensity(current_ray.wavelength);
                }
                else if let DARK(surface, material) = i.0 {
                    current_ray = material.get_next_ray(current_ray, i.1);
                    intensity = intensity * current_ray.strength;
                }
                current_ray.position = current_ray.position + current_ray.direction * 0.0001;
                continue_chance *= 0.96;
                if ptrandom::get_unit() * 0.85 > continue_chance * (1.0 - (intensity * -20.0).exp()) {
                    break;
                }
            }
            else {
                return 0.0;
            }
        }
        return 0.0;
    }
}


impl Iterator for RenderIterator<'_> {
    type Item = Photon;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.render_slice())
    }
}

