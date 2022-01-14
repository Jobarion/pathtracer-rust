use glam::Vec3;

pub struct Ray {
    pub position: Vec3,
    pub direction: Vec3,
    pub wavelength: f32,
    pub strength: f32
}

impl Ray {

    pub fn new(position: Vec3, direction: Vec3, wavelength: f32, strength: f32) -> Ray {
        Ray { position, direction, wavelength, strength }
    }
}